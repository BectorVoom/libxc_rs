//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1100/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1100<F: Float>(t14791: F, t18333: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F) -> (F,) {
    let t18334 = t14791 * t18333;
    let t18338 = t2703 * t5985;
    let t18340 = t10905 * t5989;
    let t18343 = -t14736 + t14744 + t14759 - 0.90357964994909313582e-5 * t14761 - 0.30488190661738479624e-3 * t10678 - t10687 + t10692 + 0.17149607247227894789e-2 * t2745 * t18334 - 35.0 / 108.0 * t14765 + 7.0 / 144.0 * t18338 - 7.0 / 48.0 * t18340 - 0.80031500487063509016e-2 * t14777;
    (t18343,)
}
