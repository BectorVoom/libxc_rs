//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 889/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk889<F: Float>(t14758: F, t4463: F, t12916: F, t12919: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12943: F, t12946: F, t12948: F, t12954: F) -> (F, F) {
    let t14759 = t14758 * t4463;
    let t14776 = -0.52945875e1 * t12916 + 0.94674375e0 * t12919 - 0.57386111111111111112e0 * t12922 - 0.516475e0 * t12927 - 0.68863333333333333332e0 * t12929 + 0.51647499999999999999e0 * t12931 + 0.34431666666666666666e0 * t12933 - 0.34731666666666666667e0 * t12935 + 0.20839e0 * t12937 + 0.69463333333333333335e-1 * t12939 - 0.46308888888888888889e-1 * t12943 - 0.104195e0 * t12946 - 0.103295e1 * t12948 + 0.20659e1 * t12954;
    (t14759, t14776)
}
