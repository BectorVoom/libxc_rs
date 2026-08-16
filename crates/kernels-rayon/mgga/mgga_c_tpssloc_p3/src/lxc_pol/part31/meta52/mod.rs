//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk344;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta52(t607: f64, t998: f64, t974: f64, t225: f64, t990: f64, t68: f64, t369: f64, t191: f64, t349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t999, t1000, t1003, t1004, t1005, t1008, t1009) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk344(t607, t998, t974, t225, t990, t68, t369, t191);
        let (t1010, t1011) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk345(t1009, t349, t68);
    (t999, t1000, t1003, t1004, t1005, t1008, t1009, t1010, t1011)
}
