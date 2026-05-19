//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1194/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1194<F: Float>(t34045: F, t3644: F, t3637: F, t102: F, t1563: F, t128: F, t127: F, t19349: F, t19351: F, t19355: F, t19357: F, t19359: F, t19367: F, t25828: F, t33975: F, t34039: F) -> (F, F, F, F, F, F) {
    let t48736 = F::cast_from(0.19486833333333333333e1_f64) * t34045;
    let t48737 = t3644 * t3644;
    let t48741 = t3637 * t3637;
    let t48747 = F::new(0.701526e2) * t102 * t1563 * t48737;
    let t48750 = F::new(0.1753815e2) * t102 * t128 * t48741;
    let t48752 = -F::new(0.587616e1) * t33975 + F::new(4.0) * t34039 + t48736 + F::new(0.1762848e3) * t127 * t19367 * t48737 + F::new(0.1762848e2) * t127 * t1563 * t48741 + t48747 - t19349 + t19351 + t19355 + t19357 + t19359 + t48750 + F::new(56.0) / F::new(27.0) * t25828;
    (t48736, t48737, t48741, t48747, t48750, t48752)
}
