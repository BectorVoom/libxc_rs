//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 748/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk748(t10442: f64, t1842: f64, t5117: f64, t970: f64, t1856: f64, t10585: f64, t1835: f64, t706: f64, t10593: f64, t1857: f64, t3123: f64, t5144: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11545 = t1842 * t10442;
    let t11548 = t970 * t5117;
    let t11550 = t1856 * t10442;
    let t11553 = t1835 * t10585;
    let t11556 = t706 * t10585;
    let t11559 = t1835 * t10593;
    let t11562 = t3123 * t1857;
    let t11564 = t970 * t5144;
    (t11545, t11548, t11550, t11553, t11556, t11559, t11562, t11564)
}
