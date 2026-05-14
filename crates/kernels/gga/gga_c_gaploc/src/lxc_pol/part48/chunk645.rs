//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 645/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk645<F: Float>(t1036: F, t11001: F, t13066: F, t13070: F, t13669: F, t13673: F, t13679: F, t13681: F, t13682: F, t13685: F, t13688: F, t13693: F, t13695: F, t2103: F, t3025: F, t317: F, t833: F) -> (F,) {
    let t13696 = -0.76685851907841499353e0 * t13066 + 0.23005755572352449806e1 * t833 * t13669 + 0.35750489951850426669e0 * t13673 * t317 + 0.71500979903700853338e0 * t1036 * t11001 + t13679 + t13681 - 0.21450293971110256002e1 * t3025 * t13682 + 0.23005755572352449806e2 * t833 * t13685 + 0.14300195980740170668e1 * t2103 * t13688 - t13693 + 0.76685851907841499353e0 * t13070 - t13695;
    (t13696,)
}
