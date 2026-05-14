//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1308/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1308<F: Float>(t32965: F, t415: F, t6961: F, t112502: F, t112506: F, t112508: F, t112510: F, t116220: F, t116756: F, t116762: F, t116765: F, t116768: F, t116771: F, t116773: F, t116779: F, t33002: F, t9649: F) -> (F, F) {
    let t116782 = t415 * t32965 * t6961;
    let t116784 = -0.40208333333333333335e-2 * t9649 * t116756 + 0.23280625000000000001e-2 * t33002 * t116220 - 0.33163888888888888888e-2 * t116762 + 0.22109259259259259258e-2 * t116765 + t116768 + 0.69444444444444444446e-2 * t112502 + t116771 - 0.33163888888888888888e-2 * t116773 + 0.22109259259259259258e-2 * t112506 + 0.11054629629629629629e-2 * t112508 + 0.18424382716049382715e-2 * t112510 + 0.66327777777777777776e-2 * t116779 + 0.13265555555555555555e-1 * t116782;
    (t116782, t116784)
}
