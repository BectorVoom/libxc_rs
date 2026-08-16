//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 486/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk486(t442: f64, t470: f64, t1056: f64, t1440: f64, t3796: f64, t3482: f64, t139: f64, t157: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t3797 = t470 * t442;
    let t3798 = t1056 * t1440;
    let t3799 = t3797 * t3798;
    let t3800 = t3796 * t3799;
    let t3801 = t3482 * t3800;
    let t3805 = t139 * t157 * t79;
    (t3797, t3799, t3800, t3801, t3805)
}
