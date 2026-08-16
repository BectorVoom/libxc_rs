//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 802/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk802(t12377: f64, t12396: f64, t11690: f64, t11693: f64, t11696: f64, t11698: f64, t11704: f64, t11707: f64, t11936: f64, t12340: f64, t12342: f64, t12345: f64, t12352: f64, t12353: f64, t12356: f64, t2042: f64, t2049: f64, t5527: f64, t5532: f64, t5533: f64, t5552: f64, t802: f64) -> f64 {
    let t12397 = t12377 + t12396;
    let t12399 = t12340 * t802 - 3.0_f64 * t12342 * t2049 + 6.0_f64 * t12345 * t5533 - 6.0_f64 * t12352 * t12353 + 6.0_f64 * t12356 * t5532 - t12397 * t2042 - 3.0_f64 * t5527 * t5552 - t11690 + t11693 - t11696 + t11698 + t11704 - t11707 + t11936;
    t12399
}
