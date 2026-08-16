//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 744/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk744(t108: f64, t489: f64, t119: f64, t6100: f64, t122: f64, t507: f64, t162: f64, t500: f64, t277: f64, t783: f64, t785: f64, t1610: f64, t1616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6243 = t489 * t108;
    let t6257 = t6100 * t119;
    let t6260 = 0.98171973930797904389e-1_f64 * t6257 * t122 * t507;
    let t6261 = t162 * t500;
    let t6262 = 1.0_f64 / t6261;
    let t6263 = t6262 * t277;
    let t6266 = 0.73613752582167450608e0_f64 * t783 * t785 * t6263;
    let t6268 = t783 * t1610 * t1616;
    (t6243, t6257, t6260, t6261, t6262, t6266, t6268)
}
