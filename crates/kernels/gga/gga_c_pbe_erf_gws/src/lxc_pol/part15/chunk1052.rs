//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1052/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1052<F: Float>(t2242: F, t4013: F, t13981: F, t4414: F, t22509: F, t4018: F, t2403: F, t8599: F, t2332: F, t864: F, t899: F, t907: F, t13806: F, t915: F, t2276: F, t2281: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51156 = t2242 * t4013;
    let t51162 = t4414 * t13981;
    let t51168 = t22509 * t4018;
    let t51179 = t8599 * t2403;
    let t51200 = t899 * t864 * t2332;
    let t51201 = t51200 * t907;
    let t51213 = t13806 * t915;
    let t51214 = t2276 * t51213;
    let t51215 = t51214 * t2281;
    (t51156, t51162, t51168, t51179, t51200, t51201, t51213, t51214, t51215)
}
