//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 971/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk971<F: Float>(t1216: F, t12847: F, t13190: F, t13192: F, t13194: F, t13231: F, t13238: F, t13240: F, t13242: F, t1421: F, t19401: F, t19404: F, t19409: F, t19414: F, t19418: F, t19420: F, t19425: F, t19436: F, t19441: F, t19445: F, t19451: F, t19710: F, t2110: F, t338: F, t3729: F, t5798: F) -> (F,) {
    let t19713 = -0.295669335e-2 * t1421 * t19401 - 0.14600954814814814815e-3 * t19404 - 0.65704296666666666667e-3 * t13190 - 0.19711289e-2 * t12847 * t19409 - 0.39422578e-2 * t12847 * t19414 + 0.32852148333333333333e-2 * t19418 * t19420 - 0.21901432222222222222e-2 * t19418 * t19425 - 0.19711289e-2 * t13192 + 0.492782225e-3 * t13194 - 0.8760572888888888889e-3 * t13231 - 0.2920190962962962963e-3 * t13238 + 0.43802864444444444445e-3 * t13240 + 0.73004774074074074075e-3 * t13242 + 0.295669335e-2 * t1421 * t19436 - 0.295669335e-2 * t1421 * t19441 - 0.1478346675e-2 * t1421 * t19445 - 8.0 * t1216 * t5798 + 0.26281718666666666666e-2 * t12847 * t19451 - 4.0 * t3729 * t2110 - 4.0 * t338 * t19710;
    (t19713,)
}
