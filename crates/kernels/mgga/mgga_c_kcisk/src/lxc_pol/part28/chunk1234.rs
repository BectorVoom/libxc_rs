//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1234/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1234<F: Float>(t32891: F, t33002: F, t33031: F, t33056: F, t34046: F, t34225: F, t35082: F, t35086: F, t35090: F, t35093: F, t35097: F, t35101: F, t35105: F, t35108: F, t35112: F, t35119: F, t35123: F, t9649: F, t9664: F, t9922: F) -> (F,) {
    let t35130 = -0.88437037037037037034e-2 * t35082 - 0.33163888888888888888e-2 * t35086 + 0.26805555555555555556e-2 * t34046 + t32891 - 0.33163888888888888888e-2 * t35090 + 0.22109259259259259258e-2 * t35093 - 0.20833333333333333334e-1 * t9664 * t35097 + 0.33163888888888888888e-2 * t35101 + 0.16581944444444444444e-2 * t35105 - 0.20833333333333333334e-1 * t9664 * t35108 - 0.20833333333333333334e-1 * t9664 * t35112 - 0.120625e-1 * t9649 * t35108 + 0.69444444444444444446e-2 * t33031 * t35119 + 0.26805555555555555556e-2 * t33056 * t35123 - 0.21444444444444444446e-1 * t34225 * t9922 - 0.23280625000000000001e-2 * t33002 * t35108;
    (t35130,)
}
