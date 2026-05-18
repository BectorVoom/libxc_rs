//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1074/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1074<F: Float>(t2173: F, t26717: F, t3220: F, t356: F, t303: F, t26673: F, t26677: F, t26681: F, t26685: F, t26688: F, t26692: F, t26697: F, t26703: F, t26708: F, t26712: F, t26715: F, t7687: F, t7703: F, t7706: F, t7711: F) -> (F, F, F, F) {
    let t26718 = t2173 * t26717;
    let t26720 = t356 * t3220;
    let t26721 = t303 * t26720;
    let t26725 = -F::new(0.88437037037037037034e-2) * t26673 - F::new(0.33163888888888888888e-2) * t26677 + F::new(0.46336805555555555556e-3) * t7703 * t26681 - F::new(0.18550940104166666667e-3) * t26685 * t26688 + F::new(0.12356481481481481482e-2) * t26692 * t7706 - F::new(0.30891203703703703704e-3) * t7703 * t26697 - F::new(0.13901041666666666667e-2) * t7703 * t26688 + F::new(0.61836467013888888889e-4) * t26685 * t26703 + F::new(0.16581944444444444444e-2) * t26708 + F::new(0.27636574074074074073e-2) * t26712 + F::new(0.46336805555555555556e-3) * t26715 + F::new(0.46336805555555555556e-3) * t26718 - F::new(0.55273148148148148147e-3) * t26721 + F::new(0.13901041666666666667e-2) * t7687 * t7711;
    (t26718, t26720, t26721, t26725)
}
