//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1059/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1059<F: Float>(t39667: F, t39678: F, t2211: F, t27136: F, t35256: F, t35262: F, t39650: F, t39655: F, t39657: F, t39663: F, t39672: F, t39676: F, t39682: F, t39686: F, t39690: F, t39694: F, t4985: F, t4999: F, t702: F, t72: F, t8115: F, t884: F) -> F {
    let t43096 = F::new(0.10909864661698136692e0) * t39667;
    let t43100 = F::new(0.15965655602485078085e0) * t39678;
    let t43105 = -F::new(0.2881692658299671676e-2) * t35256 - F::new(0.11974241701863808564e0) * t884 * t2211 * t27136 - F::new(0.5987120850931904282e-1) * t39650 - F::new(0.23948483403727617128e0) * t4985 * t8115 + t72 * t4999 * t702 - F::new(0.15323255961587222184e-3) * t39655 + F::new(0.20431007948782962912e-3) * t39657 - F::new(0.40911992481368012596e-1) * t39663 + t43096 - F::new(0.15965655602485078085e0) * t35262 + F::new(0.2727466165424534173e0) * t39672 + F::new(0.5454932330849068346e-1) * t39676 + t43100 + F::new(0.2727466165424534173e-1) * t39682 + F::new(0.13637330827122670865e0) * t39686 - F::new(0.8182398496273602519e-1) * t39690 + F::new(0.43639458646792546769e0) * t39694;
    t43105
}
