//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 951/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk951<F: Float>(t16826: F, t1824: F, t6750: F, t2372: F, t4623: F, t4605: F, t11401: F, t4610: F, t180: F, t4594: F, t479: F, t1887: F, t4597: F, t16304: F, t11314: F, t11316: F, t11318: F, t11320: F, t11338: F, t11400: F, t1421: F, t16784: F, t16787: F, t16790: F, t16794: F, t16798: F, t16801: F, t16807: F, t16810: F, t16814: F, t16818: F, t16823: F) -> (F, F) {
    let t16828 = t16826 * t6750 * t1824;
    let t16835 = t4623 * t2372;
    let t16836 = t16835 * t4605;
    let t16839 = t11401 * t2372;
    let t16840 = t16839 * t4610;
    let t16844 = t180 * t479 * t4594;
    let t16845 = t1887 * t4597;
    let t16846 = t16845 * t16304;
    let t16850 = -t16784 - 0.19711289e-2 * t1421 * t16787 + 0.39422577999999999999e-2 * t1421 * t16790 + 0.13140859333333333333e-2 * t1421 * t16794 + 0.492782225e-3 * t1421 * t16798 - 0.65704296666666666666e-2 * t1421 * t16801 + 0.59133867e-2 * t1421 * t16807 - 0.8760572888888888889e-3 * t16810 - 0.13140859333333333333e-2 * t1421 * t16814 - 0.65704296666666666667e-3 * t1421 * t16818 - 0.10950716111111111111e-2 * t1421 * t16823 - 0.1478346675e-2 * t1421 * t16828 - 0.2920190962962962963e-3 * t11314 + 0.43802864444444444445e-3 * t11316 + 0.73004774074074074075e-3 * t11318 - 0.19711289e-2 * t11320 + 0.98556445e-3 * t11400 * t16836 - 0.19711289e-2 * t11400 * t16840 - 0.21901432222222222222e-2 * t16844 * t16846 - 0.65704296666666666667e-3 * t11338;
    (t16844, t16850)
}
