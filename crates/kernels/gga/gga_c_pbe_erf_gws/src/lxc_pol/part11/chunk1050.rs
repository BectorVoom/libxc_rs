//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1050/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1050<F: Float>(t102: F, t128: F, t48741: F, t127: F, t1563: F, t19349: F, t19351: F, t19355: F, t19357: F, t19359: F, t19367: F, t25828: F, t33975: F, t34039: F, t48736: F, t48737: F, t48747: F) -> (F, F) {
    let t48750 = 0.1753815e2 * t102 * t128 * t48741;
    let t48752 = -0.587616e1 * t33975 + 4.0 * t34039 + t48736 + 0.1762848e3 * t127 * t19367 * t48737 + 0.1762848e2 * t127 * t1563 * t48741 + t48747 - t19349 + t19351 + t19355 + t19357 + t19359 + t48750 + 56.0 / 27.0 * t25828;
    (t48750, t48752)
}
