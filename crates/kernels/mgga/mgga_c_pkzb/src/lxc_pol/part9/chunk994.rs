//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 994/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk994<F: Float>(t1593: F, t204: F, t4994: F, t465: F, t4998: F, t5003: F, t5007: F, t5009: F, t471: F, t492: F, t574: F, t110: F, t148: F, t1503: F, t1564: F, t1569: F, t1572: F, t1581: F, t1587: F, t1608: F, t1613: F, t1615: F, t1618: F, t16190: F, t1622: F, t49: F, t4907: F, t4920: F, t4922: F, t4953: F, t4965: F, t4967: F, t4979: F, t4982: F, t5044: F, t5052: F, t5056: F, t534: F, t542: F) -> (F, F, F, F, F) {
    let t16775 = 0.71233333333333333332e-1 * t204 * t1593 * t4994;
    let t16779 = 0.3684616320282908548e2 * t204 * t465 * t4998 * t5003;
    let t16783 = 0.68734380377411894876e1 * t204 * t465 * t5007 * t5009;
    let t16787 = 0.22161481481481481481e0 * t204 * t574 * t471 * t492;
    let t16794 = -0.67471172535210825684e-1 * t204 * t574 * t534 * t542 + 0.43374325201206959368e-1 * t204 * t5056 * t1618 + 0.12842595503380418954e1 * t204 * t148 * t1503 * t1622 - 0.41096e0 * t204 * t5044 * t4907 - 0.86748650402413918736e-1 * t204 * t148 * t1613 * t1615 - 0.1301229756036208781e0 * t204 * t5052 * t4982 - 0.27397333333333333333e0 * t204 * t148 * t1569 * t1572 + 0.13218100589565368422e2 * t204 * t465 * t4965 * t4967 - 0.68493333333333333332e-1 * t204 * t1564 * t4979 + 0.38527786510141256862e1 * t204 * t465 * t4920 * t4922 - 0.21687162600603479684e-1 * t204 * t1608 * t4953 + t16775 + t16779 - t16783 + t16787 + 36.0 * t1587 * t1572 * t1581 - 0.55209406483950617283e-2 * t49 * t16190 * t110;
    (t16775, t16779, t16783, t16787, t16794)
}
