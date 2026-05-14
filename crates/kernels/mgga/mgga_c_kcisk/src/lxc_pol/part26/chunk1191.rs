//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1191/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1191<F: Float>(t34789: F, t415: F, t2718: F, t32008: F, t32087: F, t33535: F, t33555: F, t33564: F, t34759: F, t34763: F, t34768: F, t34774: F, t34777: F, t34781: F, t34784: F, t34787: F) -> (F, F) {
    let t34790 = t415 * t34789;
    let t34792 = -0.33163888888888888888e-2 * t33535 - 0.33163888888888888888e-2 * t34759 - 0.69444444444444444446e-2 * t33555 + 0.26805555555555555556e-2 * t32008 * t34763 + 0.69444444444444444446e-2 * t32087 * t34768 + 0.69444444444444444446e-2 * t32087 * t34763 + 0.33163888888888888888e-2 * t33564 - 0.10416666666666666667e-1 * t34774 * t2718 - 0.20833333333333333334e-1 * t34777 * t2718 - 0.49745833333333333332e-2 * t34781 - 0.24872916666666666666e-2 * t34784 + 0.16581944444444444444e-2 * t34787 + 0.49745833333333333332e-2 * t34790;
    (t34790, t34792)
}
