//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1068/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1068(t1593: f64, t204: f64, t4994: f64, t465: f64, t4998: f64, t5003: f64, t5007: f64, t5009: f64, t471: f64, t492: f64, t574: f64, t110: f64, t148: f64, t1503: f64, t1564: f64, t1569: f64, t1572: f64, t1581: f64, t1587: f64, t1608: f64, t1613: f64, t1615: f64, t1618: f64, t16190: f64, t1622: f64, t49: f64, t4907: f64, t4920: f64, t4922: f64, t4953: f64, t4965: f64, t4967: f64, t4979: f64, t4982: f64, t5044: f64, t5052: f64, t5056: f64, t534: f64, t542: f64) -> (f64, f64, f64, f64, f64) {
    let t16775 = 0.71233333333333333332e-1_f64 * t204 * t1593 * t4994;
    let t16779 = 0.3684616320282908548e2_f64 * t204 * t465 * t4998 * t5003;
    let t16783 = 0.68734380377411894876e1_f64 * t204 * t465 * t5007 * t5009;
    let t16787 = 0.22161481481481481481e0_f64 * t204 * t574 * t471 * t492;
    let t16794 = -0.67471172535210825684e-1_f64 * t204 * t574 * t534 * t542 + 0.43374325201206959368e-1_f64 * t204 * t5056 * t1618 + 0.12842595503380418954e1_f64 * t204 * t148 * t1503 * t1622 - 0.41096e0_f64 * t204 * t5044 * t4907 - 0.86748650402413918736e-1_f64 * t204 * t148 * t1613 * t1615 - 0.1301229756036208781e0_f64 * t204 * t5052 * t4982 - 0.27397333333333333333e0_f64 * t204 * t148 * t1569 * t1572 + 0.13218100589565368422e2_f64 * t204 * t465 * t4965 * t4967 - 0.68493333333333333332e-1_f64 * t204 * t1564 * t4979 + 0.38527786510141256862e1_f64 * t204 * t465 * t4920 * t4922 - 0.21687162600603479684e-1_f64 * t204 * t1608 * t4953 + t16775 + t16779 - t16783 + t16787 + 36.0_f64 * t1587 * t1572 * t1581 - 0.55209406483950617283e-2_f64 * t49 * t16190 * t110;
    (t16775, t16779, t16783, t16787, t16794)
}
