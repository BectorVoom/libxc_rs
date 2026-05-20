//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3035/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035<F: Float>(t11044: F, t14983: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t14991: F, t41066: F, t10982: F, t1568: F, t9646: F) -> (F, F, F, F, F, F) {
    let t51231 = t11044 * t14983;
    let t51233 = t11044 * t14485;
    let t51237 = t9303 * t15014;
    let t51239 = t14987 * t10510;
    let t51241 = t41066 * t14991;
    let t51246 = t9646 * t1568 * t10982;
    (t51231, t51233, t51237, t51239, t51241, t51246)
}
