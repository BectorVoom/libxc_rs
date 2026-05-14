//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 995/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk995<F: Float>(t6491: F, t702: F, t289: F, t36804: F, t36809: F, t38079: F, t38080: F, t43937: F, t43948: F, t47721: F, t47723: F, t47725: F, t47727: F, t47729: F, t47735: F, t47737: F, t47740: F, t47743: F, t47745: F, t47747: F) -> (F,) {
    let t49655 = t6491 * t702;
    let t49666 = 0.5454932330849068346e-1 * t47721 + 0.5454932330849068346e-1 * t47723 + 0.40911992481368012595e-1 * t47725 - 0.5454932330849068346e-1 * t47727 - 0.40911992481368012595e-1 * t47729 - 0.2363e1 * t289 * t49655 - t38079 + t38080 + 0.162600798888400151e-2 * t36804 + 0.162600798888400151e-2 * t36809 - t43937 - 0.71845450211182851384e0 * t47735 - 0.17961362552795712846e0 * t47737 - 0.17961362552795712846e0 * t47740 - 0.17961362552795712846e0 * t47743 + t43948 - 0.40911992481368012596e-1 * t47745 - 0.40911992481368012596e-1 * t47747;
    (t49666,)
}
