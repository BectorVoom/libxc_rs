//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1089/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1089<F: Float>(t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F, t232: F, t24677: F, t7680: F, t774: F) -> (F, F) {
    let t24678 = 0.18467901234567901234e0 * t23682;
    let t24690 = t24678 - 0.47488888888888888888e-1 * t23620 - 0.31659259259259259258e-1 * t23622 + 0.23744444444444444444e-1 * t23624 + 0.26382716049382716049e-1 * t23626 - 0.52765432098765432099e-1 * t23630 - 0.17808333333333333333e-1 * t23633 + 0.73871604938271604937e-1 * t23635 - 0.94977777777777777776e-1 * t23637 + 0.23744444444444444444e0 * t23640 + 0.10685e0 * t23644 + 0.14246666666666666667e0 * t23660;
    let t24693 = 0.62182e-1 * (t24677 + t24690) * t232;
    let t24694 = t774 * t7680;
    (t24693, t24694)
}
