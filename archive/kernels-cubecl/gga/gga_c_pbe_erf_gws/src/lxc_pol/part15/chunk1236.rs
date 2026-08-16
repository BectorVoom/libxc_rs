//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1236/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1236<F: Float>(t20154: F, t3067: F, t4164: F, t810: F, t14629: F, t4414: F, t14624: F, t9270: F, t14767: F, t2373: F, t1113: F, t13781: F, t2352: F, t3972: F, t824: F) -> (F, F, F, F, F) {
    let t53083 = t20154 * t3067 * t4164 * t810;
    let t53093 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t14629;
    let t53099 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14624;
    let t53126 = t14767 * t2373;
    let t53131 = t3972 * t13781 * t1113 * t824 * t2352;
    (t53083, t53093, t53099, t53126, t53131)
}
