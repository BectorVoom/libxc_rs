//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1302/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1302<F: Float>(t14437: F, t3083: F, t3733: F, t54580: F, t13781: F, t3808: F, t3972: F, t52000: F, t13782: F, t3861: F, t2306: F, t3037: F, t3975: F, t9385: F) -> (F, F, F, F, F) {
    let t56620 = t3083 * t14437;
    let t56626 = t54580 * t3733;
    let t56638 = t3972 * t13781 * t3808 * t52000;
    let t56642 = t3972 * t13781 * t3861 * t13782;
    let t56647 = t3972 * t3975 * t9385 * t2306 * t3037;
    (t56620, t56626, t56638, t56642, t56647)
}
