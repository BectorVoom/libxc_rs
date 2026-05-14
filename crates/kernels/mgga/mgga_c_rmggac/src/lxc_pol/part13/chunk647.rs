//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 647/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk647<F: Float>(t8729: F, t8731: F, t8733: F, t8739: F, t8744: F, t8748: F, t8752: F, t8755: F, t9457: F, t9458: F, t9460: F, t7597: F, t7618: F, t7620: F, t8129: F, t8759: F, t8762: F, t8765: F, t8767: F, t8769: F, t8771: F, t8773: F) -> (F, F) {
    let t9465 = 0.2993560425465952141e-1 * t8729 - 0.19957069503106347607e-1 * t8731 - 0.19957069503106347607e-1 * t8733 + t9457 - t9458 - 0.79828278012425390427e-1 * t8739 + t9460 - 0.2727466165424534173e-1 * t8744 + 0.45457769423742236216e-1 * t8748 + 0.9072038638458063915e-3 * t8752 - 0.12700854093841289481e-2 * t8755;
    let t9477 = -0.12700854093841289481e-2 * t8759 + 0.16934472125121719308e-2 * t8762 + 0.13637330827122670865e-1 * t8765 - 0.2727466165424534173e-1 * t8767 + t8129 + 0.59871208509319042821e-1 * t8769 - 0.26552308210121162678e-2 * t8771 + 0.39828462315181744017e-2 * t8773 + 0.53218852008283593618e-1 * t7597 - 0.79828278012425390427e-1 * t7618 + 0.17701538806747441786e-2 * t7620;
    (t9465, t9477)
}
