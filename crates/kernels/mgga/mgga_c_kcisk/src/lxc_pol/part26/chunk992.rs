//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 992/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk992<F: Float>(t3592: F, t442: F, t25915: F, t1417: F, t7866: F, t1216: F, t12847: F, t13238: F, t1421: F, t19322: F, t19324: F, t19380: F, t19386: F, t19388: F, t19404: F, t19418: F, t2110: F, t25906: F, t26710: F, t26712: F, t26714: F, t26719: F, t26723: F, t26727: F, t26730: F, t26734: F, t338: F, t5798: F, t7828: F) -> (F,) {
    let t26737 = t3592 * t442;
    let t26738 = t26737 * t25915;
    let t26746 = t1417 * t7866;
    let t26748 = -t19322 + t19324 - 4.0 * t338 * t25906 + 0.13140859333333333333e-2 * t26710 + 0.98556445e-3 * t26712 - 0.65704296666666666667e-3 * t26714 - t19380 - t19386 + t19388 - 0.2920190962962962963e-3 * t19404 - 0.1478346675e-2 * t1421 * t26719 + 0.19711289e-2 * t1421 * t26723 - 0.39422578e-2 * t12847 * t26727 + 0.32852148333333333333e-2 * t19418 * t26730 - 0.19711289e-2 * t12847 * t26734 + 0.26281718666666666666e-2 * t12847 * t26738 - 4.0 * t1216 * t7828 - 0.14600954814814814815e-3 * t13238 - 8.0 * t2110 * t5798 + 0.492782225e-3 * t26746;
    (t26748,)
}
