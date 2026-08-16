//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1194/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1194(t34045: f64, t3644: f64, t3637: f64, t102: f64, t1563: f64, t128: f64, t127: f64, t19349: f64, t19351: f64, t19355: f64, t19357: f64, t19359: f64, t19367: f64, t25828: f64, t33975: f64, t34039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48736 = 0.19486833333333333333e1_f64 * t34045;
    let t48737 = t3644 * t3644;
    let t48741 = t3637 * t3637;
    let t48747 = 0.701526e2_f64 * t102 * t1563 * t48737;
    let t48750 = 0.1753815e2_f64 * t102 * t128 * t48741;
    let t48752 = -0.587616e1_f64 * t33975 + 4.0_f64 * t34039 + t48736 + 0.1762848e3_f64 * t127 * t19367 * t48737 + 0.1762848e2_f64 * t127 * t1563 * t48741 + t48747 - t19349 + t19351 + t19355 + t19357 + t19359 + t48750 + 56.0_f64 / 27.0_f64 * t25828;
    (t48736, t48737, t48741, t48747, t48750, t48752)
}
