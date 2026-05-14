//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1441/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1441<F: Float>(t112661: F, t112663: F, t113123: F, t117133: F, t118439: F, t118450: F, t118455: F, t118466: F, t121921: F, t121928: F, t121941: F, t121945: F, t122573: F, t122755: F, t122818: F, t123004: F, t34419: F) -> (F,) {
    let t123207 = -0.13402777777777777778e-2 * t113123 * t122755 + t118439 - 0.61905925925925925924e-2 * t121921 + 0.38691203703703703703e-3 * t112661 - 0.25794135802469135802e-3 * t112663 + 0.11607361111111111111e-2 * t121928 - 0.30952962962962962962e-2 * t117133 - 0.92858888888888888886e-2 * t121941 - 0.41270617283950617283e-2 * t121945 - 0.35740740740740740741e-2 * t118450 + t118455 - 0.13402777777777777778e-2 * t113123 * t123004 + 0.77160493827160493827e-3 * t118466 + 0.116403125e-2 * t34419 * t122573 + 0.69841875000000000001e-2 * t34419 * t122818;
    (t123207,)
}
