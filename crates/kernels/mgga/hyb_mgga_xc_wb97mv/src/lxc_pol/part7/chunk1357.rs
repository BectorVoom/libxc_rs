//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1357/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1357<F: Float>(t10080: F, t9978: F, t13473: F, t9831: F, t7899: F, t10177: F, t11775: F, t11939: F, t24237: F, t24260: F, t28338: F, t28347: F, t28621: F, t28666: F, t28682: F, t28686: F, t28701: F, t28705: F, t28729: F, t32842: F, t33038: F, t33147: F, t33177: F, t33292: F, t3799: F, t7833: F, t7897: F) -> (F,) {
    let t33337 = t10080 * t9978;
    let t33346 = t13473 * t9831;
    let t33349 = t13473 * t7899;
    let t33360 = -0.384e-3 * t28729 * t11939 + 0.704e-3 * t7897 * t32842 + 0.1512e2 * t28666 * t3799 * t9978 + 0.576e1 * t24237 * t7833 * t11775 - 0.384e0 * t28705 * t33337 - 0.3072e-5 * t7897 * t33292 - 0.384e0 * t24260 * t33038 - 0.8064e1 * t28701 * t33337 - 0.6144e-5 * t28686 * t33346 + 0.9216e-5 * t28347 * t33349 - 0.17066666666666666667e-4 * t10177 * t33177 - 0.79999999999999999999e0 * t28621 * t33147 - 0.55296e-4 * t28682 * t33346 + 0.64512e-4 * t28338 * t33349;
    (t33360,)
}
