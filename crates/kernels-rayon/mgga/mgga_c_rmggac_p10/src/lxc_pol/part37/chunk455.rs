//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 455/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk455(t8737: f64, t8741: f64, t8729: f64, t8731: f64, t8733: f64, t8739: f64, t8744: f64, t8748: f64, t8752: f64, t8755: f64, t9457: f64, t7597: f64, t7618: f64, t7620: f64, t8129: f64, t8759: f64, t8762: f64, t8765: f64, t8767: f64, t8769: f64, t8771: f64, t8773: f64) -> (f64, f64) {
    let t9458 = 0.21241846568096930142e-2_f64 * t8737;
    let t9460 = 0.53218852008283593619e-1_f64 * t8741;
    let t9465 = 0.2993560425465952141e-1_f64 * t8729 - 0.19957069503106347607e-1_f64 * t8731 - 0.19957069503106347607e-1_f64 * t8733 + t9457 - t9458 - 0.79828278012425390427e-1_f64 * t8739 + t9460 - 0.2727466165424534173e-1_f64 * t8744 + 0.45457769423742236216e-1_f64 * t8748 + 0.9072038638458063915e-3_f64 * t8752 - 0.12700854093841289481e-2_f64 * t8755;
    let t9477 = -0.12700854093841289481e-2_f64 * t8759 + 0.16934472125121719308e-2_f64 * t8762 + 0.13637330827122670865e-1_f64 * t8765 - 0.2727466165424534173e-1_f64 * t8767 + t8129 + 0.59871208509319042821e-1_f64 * t8769 - 0.26552308210121162678e-2_f64 * t8771 + 0.39828462315181744017e-2_f64 * t8773 + 0.53218852008283593618e-1_f64 * t7597 - 0.79828278012425390427e-1_f64 * t7618 + 0.17701538806747441786e-2_f64 * t7620;
    (t9465, t9477)
}
