//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 990/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk990<F: Float>(t18102: F, t894: F, t18019: F, t3245: F, t18030: F, t3235: F, t15786: F, t17921: F, t15274: F, t18023: F, t3087: F, t914: F) -> (F, F, F, F, F, F, F) {
    let t18103 = t894 * t18102;
    let t18106 = t3245 * t18019;
    let t18114 = t3235 * t18030;
    let t18117 = t15786 * t17921;
    let t18120 = t15786 * t15274;
    let t18130 = t3087 * t18023;
    let t18131 = t914 * t18130;
    (t18103, t18106, t18114, t18117, t18120, t18130, t18131)
}
