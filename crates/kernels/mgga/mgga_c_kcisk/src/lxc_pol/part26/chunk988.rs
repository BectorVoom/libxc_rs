//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 988/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk988<F: Float>(t2083: F, t2191: F, t1175: F, t19244: F, t1364: F, t19351: F, t3539: F, t7740: F, t3544: F, t25441: F, t5907: F, t12845: F, t12847: F, t1421: F, t19150: F, t19163: F, t19418: F, t26600: F, t26602: F, t26606: F, t26610: F, t26613: F, t26619: F, t26623: F, t26626: F, t26629: F, t26632: F, t26636: F, t26641: F) -> (F,) {
    let t26644 = t2083 * t2191;
    let t26646 = t19244 * t26644 * t1175;
    let t26650 = t19351 * t26644 * t1364;
    let t26654 = t3539 * t7740 * t1175;
    let t26659 = t3544 * t7740 * t1364;
    let t26662 = t5907 * t25441;
    let t26665 = -0.19711289e-2 * t26600 + 0.43802864444444444445e-3 * t26602 + 0.295669335e-2 * t1421 * t26606 - 0.59133867e-2 * t1421 * t26610 - 0.19711289e-2 * t12847 * t26613 + 0.98556445e-3 * t12847 * t26619 - 0.19711289e-2 * t12847 * t26623 + t12845 + t19150 + 0.26281718666666666666e-2 * t12847 * t26626 - 0.21901432222222222222e-2 * t19418 * t26629 + 0.13140859333333333333e-2 * t26632 - 0.65704296666666666667e-3 * t1421 * t26636 + 0.492782225e-3 * t1421 * t26641 - 0.1478346675e-2 * t1421 * t26646 + 0.59133867e-2 * t1421 * t26650 - 0.19711289e-2 * t1421 * t26654 - 0.65704296666666666667e-3 * t19163 + 0.13140859333333333333e-2 * t1421 * t26659 + 0.39422577999999999999e-2 * t1421 * t26662;
    (t26665,)
}
