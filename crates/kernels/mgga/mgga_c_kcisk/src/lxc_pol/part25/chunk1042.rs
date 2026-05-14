//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1042/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1042<F: Float>(t4790: F, t6874: F, t1683: F, t1973: F, t7490: F, t2605: F, t5392: F, t5374: F, t7493: F, t5400: F, t7489: F, t12061: F, t2604: F, t2609: F, t4762: F, t11999: F, t12018: F, t12059: F, t12084: F, t12114: F, t16363: F, t16441: F, t5373: F, t5398: F, t5415: F, t7472: F, t7494: F, t7503: F) -> (F,) {
    let t18565 = t6874 * t4790;
    let t18566 = t18565 * t1683;
    let t18573 = t7490 * t1973;
    let t18576 = t2605 * t5392;
    let t18579 = t7493 * t5374;
    let t18582 = t7489 * t5400;
    let t18583 = t18582 * t1973;
    let t18586 = t7493 * t5392;
    let t18589 = t2604 * t12061;
    let t18590 = t18589 * t5374;
    let t18593 = t2609 * t4762;
    let t18598 = -t16363 - t16441 + 0.34631511798751726598e2 * t5415 * t18566 - 4.0 * t11999 * t7472 + 0.64329366355741395948e2 * t12114 * t7494 - 4.0 * t5373 * t18573 - 2.0 * t5373 * t18576 - 0.19298809906722418785e3 * t12018 * t18579 + 0.64329366355741395948e2 * t5398 * t18583 + 0.32164683177870697974e2 * t5398 * t18586 + 0.20691336878655965246e4 * t12059 * t18590 + 0.35089340384731224426e1 * t5415 * t18593 - 0.23392893589820816284e1 * t12084 * t7503;
    (t18598,)
}
