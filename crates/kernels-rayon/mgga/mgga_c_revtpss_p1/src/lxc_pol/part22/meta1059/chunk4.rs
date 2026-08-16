//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3767/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767(t17729: f64, t20922: f64, t44425: f64, t17396: f64, t17617: f64, t1222: f64, t6658: f64, t697: f64, t6662: f64, t12268: f64, t12787: f64, t15936: f64, t17580: f64, t17625: f64, t17730: f64, t1774: f64, t20317: f64, t3626: f64, t5354: f64, t56953: f64, t57147: f64, t59162: f64, t59338: f64, t59349: f64, t59351: f64, t59353: f64) -> f64 {
    let t71908 = t17729 * t44425 * t20922;
    let t71920 = t17396 * t17617;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t71936 = -0.19055119163586549765e-3_f64 * t59338 - 0.6351706387862183255e-3_f64 * t71908 - 0.85748036236139473944e-3_f64 * t59162 * t17580 + 0.57165357490759649296e-3_f64 * t17729 * t3626 * t20317 * t17730 + 0.45732285992607719436e-2_f64 * t56953 * t5354 - 0.45732285992607719436e-2_f64 * t57147 * t17625 + 0.30488190661738479624e-2_f64 * t71920 - 0.28582678745379824648e-2_f64 * t17729 * t12787 * t1774 * t12268 * t15936 + t71928 / 1296.0_f64 + t71931 / 648.0_f64 - 0.28582678745379824648e-3_f64 * t59349 - 0.28582678745379824648e-3_f64 * t59351 + 0.57165357490759649296e-3_f64 * t59353;
    t71936
}
