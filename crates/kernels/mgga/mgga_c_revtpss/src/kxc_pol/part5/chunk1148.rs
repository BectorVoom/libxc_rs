//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1148/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1148<F: Float>(t14494: F, t6035: F, t14791: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F) -> F {
    let t18333 = t14494 * t6035;
    let t18334 = t14791 * t18333;
    let t18338 = t2703 * t5985;
    let t18340 = t10905 * t5989;
    let t18343 = -t14736 + t14744 + t14759 - F::new(0.90357964994909313582e-5) * t14761 - F::new(0.30488190661738479624e-3) * t10678 - t10687 + t10692 + F::new(0.17149607247227894789e-2) * t2745 * t18334 - F::new(35.0) / F::new(108.0) * t14765 + F::new(7.0) / F::new(144.0) * t18338 - F::new(7.0) / F::new(48.0) * t18340 - F::new(0.80031500487063509016e-2) * t14777;
    t18343
}
