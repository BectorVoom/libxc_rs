//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1059/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1059(t39667: f64, t39678: f64, t2211: f64, t27136: f64, t35256: f64, t35262: f64, t39650: f64, t39655: f64, t39657: f64, t39663: f64, t39672: f64, t39676: f64, t39682: f64, t39686: f64, t39690: f64, t39694: f64, t4985: f64, t4999: f64, t702: f64, t72: f64, t8115: f64, t884: f64) -> f64 {
    let t43096 = 0.10909864661698136692e0_f64 * t39667;
    let t43100 = 0.15965655602485078085e0_f64 * t39678;
    let t43105 = -0.2881692658299671676e-2_f64 * t35256 - 0.11974241701863808564e0_f64 * t884 * t2211 * t27136 - 0.5987120850931904282e-1_f64 * t39650 - 0.23948483403727617128e0_f64 * t4985 * t8115 + t72 * t4999 * t702 - 0.15323255961587222184e-3_f64 * t39655 + 0.20431007948782962912e-3_f64 * t39657 - 0.40911992481368012596e-1_f64 * t39663 + t43096 - 0.15965655602485078085e0_f64 * t35262 + 0.2727466165424534173e0_f64 * t39672 + 0.5454932330849068346e-1_f64 * t39676 + t43100 + 0.2727466165424534173e-1_f64 * t39682 + 0.13637330827122670865e0_f64 * t39686 - 0.8182398496273602519e-1_f64 * t39690 + 0.43639458646792546769e0_f64 * t39694;
    t43105
}
