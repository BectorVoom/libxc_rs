//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 550/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk550<F: Float>(t2874: F, t2876: F, t2882: F, t2887: F, t2892: F, t2895: F, t2897: F, t2900: F, t2904: F, t2907: F, t3477: F, t1104: F, t575: F, t1112: F, t687: F, t2970: F, t2976: F, t2984: F, t2988: F, t2991: F, t3002: F, t3009: F, t3014: F, t3019: F, t3023: F, t3025: F) -> (F, F, F, F) {
    let t3478 = -0.3475929712541504153e-2 * t2874 + 0.20855578275249024918e-2 * t2876 - 0.20855578275249024918e-2 * t2882 - 0.69518594250830083059e-4 * t2887 + 0.12360406057797588768e-3 * t2892 + 0.20855578275249024918e-2 * t2895 + 0.27517776890953574545e-3 * t2897 - 0.20855578275249024918e-2 * t2900 - 0.26319242435966565832e-3 * t2904 + 0.60736713313768998073e-4 * t2907 + t3477;
    let t3480 = t1104 * t575;
    let t3483 = t1112 * t687;
    let t3497 = 0.10821235962619981449e-3 * t2970 + 0.12163329537032409896e-2 * t2976 - 0.20241536458333333335e-4 * t2984 + 0.17376185052903442709e-3 * t2988 + 0.17376185052903442709e-3 * t2991 + 0.16882592796244404291e-6 * t3002 + 0.33765185592488808582e-6 * t3009 - 0.50680539737635041235e-4 * t3014 - 0.14492726735651760868e-5 * t3019 + 0.28985453471303521736e-5 * t3023 - 0.16908181191593721013e-4 * t3025;
    (t3478, t3480, t3483, t3497)
}
