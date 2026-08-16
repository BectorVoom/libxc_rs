//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 535/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk535(t1338: f64, t1372: f64, t193: f64, t532: f64, t1388: f64, t1390: f64, t112: f64, t1395: f64) -> (f64, f64, f64, f64) {
    let t3901 = t1338 * t1372;
    let t3918 = t193 * t532;
    let t3919 = t1388 * t1390;
    let t3938 = t1395 * t112;
    (t3901, t3918, t3919, t3938)
}
