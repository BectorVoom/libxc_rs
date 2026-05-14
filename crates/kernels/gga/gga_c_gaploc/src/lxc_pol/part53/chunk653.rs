//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 653/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk653<F: Float>(t13073: F, t13079: F, t13113: F, t13114: F, t13115: F, t13116: F, t13117: F, t13120: F, t13886: F, t13890: F, t13893: F, t13895: F, t13124: F, t13132: F, t13138: F, t13140: F, t13144: F, t13152: F, t13156: F, t13160: F, t13163: F, t13901: F, t13904: F, t13906: F) -> (F, F) {
    let t14511 = -t13886 - t13890 - 0.29792074959875355558e-1 * t13893 + 0.29792074959875355558e-1 * t13895 - 0.89376224879626066674e-1 * t13073 + t13079 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120;
    let t14515 = 0.14300195980740170668e1 * t13901 - 0.21450293971110256002e1 * t13904 + 0.71500979903700853338e0 * t13906 + t13124 - t13132 + t13138 + t13140 + t13144 - t13152 + t13156 - t13160 - t13163;
    (t14511, t14515)
}
