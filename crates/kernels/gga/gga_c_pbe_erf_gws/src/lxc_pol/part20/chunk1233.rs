//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1233/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1233<F: Float>(t53060: F, t1144: F, t13928: F, t4386: F, t20154: F, t3067: F, t4164: F, t810: F, t14629: F, t4414: F, t14624: F, t9270: F) -> (F, F, F, F, F) {
    let t53061 = F::new(7.0) / F::new(576.0) * t53060;
    let t53075 = t4386 * t1144 * t13928;
    let t53083 = t20154 * t3067 * t4164 * t810;
    let t53093 = F::new(7.0) / F::new(72.0) * t4414 * t14629;
    let t53099 = F::new(7.0) / F::new(72.0) * t9270 * t14624;
    (t53061, t53075, t53083, t53093, t53099)
}
