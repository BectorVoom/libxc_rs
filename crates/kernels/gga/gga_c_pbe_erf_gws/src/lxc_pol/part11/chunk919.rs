//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 919/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk919<F: Float>(t18753: F, t18801: F, t18839: F, t18910: F, t40: F, t60: F, t18639: F, t18865: F, t470: F, t4737: F, t39: F, t55: F, t59: F, t87: F) -> (F, F, F) {
    let t18914 = t40 * t60 * (t18753 + t18801 + t18839 + t18910);
    let t18920 = F::cast_from(0.12304676425209353917e5_f64) * t470 * t18865 * t18639 * t4737;
    let t18924 = F::new(24.0) * t39 * t55 * t59 * t87;
    (t18914, t18920, t18924)
}
