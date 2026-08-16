//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 769/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk769(t11812: f64, t7310: f64, t10777: f64, t196: f64, t10585: f64, t7370: f64, t1849: f64, t1860: f64, t1919: f64, t3290: f64, t10441: f64, t5249: f64) -> (f64, f64, f64, f64, f64) {
    let t11813 = t7310 * t11812;
    let t11815 = t10777 * t196;
    let t11818 = t7370 * t10585;
    let t11821 = t1860 * t1849;
    let t11823 = t1919 * t11821 * t3290;
    let t11827 = t1919 * t5249 * t10441;
    (t11813, t11815, t11818, t11823, t11827)
}
