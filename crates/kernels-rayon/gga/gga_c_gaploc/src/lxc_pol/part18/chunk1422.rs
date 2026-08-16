//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1422/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1422(t35112: f64, t2365: f64, t25580: f64, t4391: f64, t10216: f64, t10584: f64, t1424: f64, t1441: f64, t1641: f64, t31623: f64, t31624: f64, t3403: f64, t3415: f64, t35075: f64, t35090: f64, t35094: f64, t35097: f64, t35100: f64, t35104: f64, t35106: f64, t35110: f64, t4634: f64, t4637: f64, t531: f64, t557: f64, t568: f64, t590: f64, t597: f64, t600: f64) -> f64 {
    let t35113 = 0.59584149919750711116e-1_f64 * t35112;
    let t35115 = t4391 * t2365 * t25580;
    let t35116 = 0.29792074959875355558e-1_f64 * t35115;
    let t35117 = 0.1022478025437886658e1_f64 * t1441 * t10216 * t590 - t35075 - 0.35750489951850426669e0_f64 * t557 * t531 * t31624 - 0.23005755572352449806e1_f64 * t4634 * t3403 - 0.46011511144704899612e1_f64 * t1641 * t10584 + 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t31623 + 0.23005755572352449806e1_f64 * t4637 * t3415 - t35090 - t35094 - t35097 + t35100 - t35104 - 0.79445533226334281486e-1_f64 * t35106 * t1424 + t35110 + t35113 + t35116;
    t35117
}
