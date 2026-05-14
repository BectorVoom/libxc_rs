//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 563/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk563<F: Float>(t2913: F, t2916: F, t2926: F, t2930: F, t2934: F, t2939: F, t2943: F, t2946: F, t2949: F, t2955: F, t2959: F, t2874: F, t2876: F, t2882: F, t2887: F, t2892: F, t2895: F, t2897: F, t2900: F, t2904: F, t2907: F) -> (F,) {
    let t3477 = -0.60736713313768998073e-4 * t2913 - 0.20245571104589666024e-4 * t2916 + 0.29524791194193262952e-5 * t2926 - 0.60736713313768998073e-4 * t2930 - 0.43449121406768801913e-4 * t2934 + 0.43449121406768801913e-4 * t2939 + 0.43449121406768801913e-5 * t2943 - 0.77252537861234929801e-5 * t2946 - 0.43449121406768801913e-4 * t2949 - 0.12672660410307567225e-4 * t2955 + 0.43449121406768801913e-4 * t2959;
    let t3478 = -0.3475929712541504153e-2 * t2874 + 0.20855578275249024918e-2 * t2876 - 0.20855578275249024918e-2 * t2882 - 0.69518594250830083059e-4 * t2887 + 0.12360406057797588768e-3 * t2892 + 0.20855578275249024918e-2 * t2895 + 0.27517776890953574545e-3 * t2897 - 0.20855578275249024918e-2 * t2900 - 0.26319242435966565832e-3 * t2904 + 0.60736713313768998073e-4 * t2907 + t3477;
    (t3478,)
}
