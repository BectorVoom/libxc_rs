//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 719/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk719<F: Float>(t13870: F, t808: F, t568: F, t813: F, t836: F, t833: F, t314: F, t313: F, t317: F, t739: F, t531: F, t797: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13871 = t808 * t13870;
    let t13872 = t568 * t13871;
    let t13874 = F::new(0.23005755572352449806e1) * t813 * t13872;
    let t13875 = t836 * t13870;
    let t13876 = t568 * t13875;
    let t13878 = F::new(0.23005755572352449806e1) * t833 * t13876;
    let t13879 = t314 * t13870;
    let t13880 = t313 * t13879;
    let t13882 = F::new(0.35750489951850426669e0) * t13880 * t317;
    let t13883 = t739 * t13870;
    let t13884 = t531 * t13883;
    let t13886 = F::new(0.35750489951850426669e0) * t797 * t13884;
    (t13871, t13872, t13874, t13875, t13876, t13878, t13879, t13880, t13882, t13883, t13884, t13886)
}
