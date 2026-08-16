//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 479/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk479(t1343: f64, t3748: f64, t1342: f64, t3512: f64, t1339: f64, t1341: f64, t3583: f64, t1340: f64, t1336: f64, t140: f64, t3529: f64, t3575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3749 = t3748 * t1343;
    let t3751 = t3512 * t1342;
    let t3752 = t1339 * t3751;
    let t3754 = t1341 * t3583;
    let t3755 = t1340 * t3754;
    let t3756 = t1339 * t3755;
    let t3759 = t140 * t1336 * t3529;
    let t3760 = t1341 * t3575;
    (t3749, t3751, t3752, t3754, t3755, t3756, t3759, t3760)
}
