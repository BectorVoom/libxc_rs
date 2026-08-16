//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1069/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1069<F: Float>(t1143: F, t2416: F, t1105: F, t2053: F, t944: F, t4058: F, t945: F, t1172: F, t318: F, t2182: F, t3944: F, t810: F) -> (F, F, F, F, F, F, F) {
    let t12213 = t1143 * t2416;
    let t12275 = t2053 * t1105;
    let t12276 = t12275 * t944;
    let t13751 = t4058 * t945;
    let t13756 = t1172 * t318;
    let t13757 = t3944 * t2182;
    let t13760 = t13751 * t810;
    (t12213, t12275, t12276, t13751, t13756, t13757, t13760)
}
