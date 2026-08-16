//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta142(t3040: f64, t3131: f64, t1021: f64, t248: f64, t135: f64, t999: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3132, t3134, t3139, t3140, t3142, t3143, t3146) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk793(t3040, t3131, t1021, t248, t135, t999, t973, t2250, t998, t974, t2770, t2978);
    (t3132, t3134, t3139, t3140, t3142, t3143, t3146)
}
