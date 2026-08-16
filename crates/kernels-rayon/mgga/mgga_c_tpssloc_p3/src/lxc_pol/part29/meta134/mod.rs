//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk767;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk768;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta134(t2770: f64, t344: f64, t2244: f64, t2979: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t984: f64, t343: f64, t883: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t2981, t2982, t2986) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk767(t2770, t344, t2244, t2979, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk768(t60, t976);
        let (t2988, t2989, t2990) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk769(t2987, t984, t343, t883, t607);
    (t2981, t2982, t2986, t2987, t2988, t2989, t2990)
}
