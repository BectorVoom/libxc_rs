//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1313/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1313<F: Float>(t113702: F, t113710: F, t113719: F, t113747: F, t114664: F, t118785: F, t118866: F, t118869: F, t118872: F, t118875: F, t118878: F, t118882: F, t118891: F, t32008: F, t32022: F, t32087: F, t33417: F, t34697: F) -> (F,) {
    let t118894 = -0.92592592592592592594e-2 * t114664 * t33417 + 0.55273148148148148147e-3 * t118866 + t113702 - 0.16581944444444444444e-2 * t118869 + 0.11054629629629629629e-2 * t118872 - t113710 - 0.36848765432098765431e-3 * t118875 + 0.33163888888888888888e-2 * t118878 + 0.61728395061728395061e-2 * t113719 - 0.69444444444444444446e-2 * t32087 * t118882 + t113747 - 0.27777777777777777779e-1 * t32022 * t34697 + 0.34722222222222222223e-2 * t32087 * t118785 - 0.80416666666666666668e-2 * t32008 * t118891;
    (t118894,)
}
