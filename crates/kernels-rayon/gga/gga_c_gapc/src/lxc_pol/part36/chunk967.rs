//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 967/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk967(t11695: f64, t3225: f64, t773: f64, t826: f64, t10264: f64, t3212: f64, t3724: f64, t3209: f64, t3765: f64, t7553: f64, t3679: f64, t7557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11696 = t3225 * t11695;
    let t11698 = t826 * t773;
    let t11699 = t10264 * t11698;
    let t11701 = t3212 * t3724;
    let t11703 = t3209 * t3724;
    let t11728 = t7553 * t3765;
    let t11730 = t3679 * t7557;
    (t11696, t11698, t11699, t11701, t11703, t11728, t11730)
}
