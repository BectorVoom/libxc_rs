//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1071/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1071<F: Float>(t14180: F, t4386: F, t892: F, t14280: F, t840: F, t2242: F, t4094: F, t4083: F, t4453: F, t51977: F, t4113: F, t1208: F, t6729: F, t1206: F, t2100: F, t353: F, t859: F) -> (F, F, F, F, F, F, F, F) {
    let t52542 = t4386 * t892 * t14180;
    let t52551 = t840 * t14280;
    let t52560 = t2242 * t4094;
    let t52562 = t4453 * t4083;
    let t52582 = 455.0 / 648.0 * t51977;
    let t52586 = t2242 * t4113;
    let t52589 = 455.0 / 1296.0 * t6729 * t1208;
    let t52600 = t859 * t353 * t1206 * t2100;
    (t52542, t52551, t52560, t52562, t52582, t52586, t52589, t52600)
}
