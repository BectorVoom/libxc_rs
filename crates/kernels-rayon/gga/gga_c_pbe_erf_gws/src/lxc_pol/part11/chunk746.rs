//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 746/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk746(t12323: f64, t171: f64, t9763: f64, t6968: f64, t7986: f64, t10017: f64, t7988: f64, t7990: f64, t4499: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12324 = t171 * t12323;
    let t12332 = 0.54934665110259479823e-3_f64 * t9763;
    let t12333 = 0.32530742648344572643e-1_f64 * t6968;
    let t12334 = 60.0_f64 * t7986;
    let t12335 = 3.0_f64 * t10017;
    let t12336 = 96.0_f64 * t7988;
    let t12337 = 24.0_f64 * t7990;
    let t12338 = -t12332 - t4499 + t4503 - t4506 - t4513 + t4539 + t4542 + t12333 + t12334 + t12335 + t12336 - t12337;
    (t12324, t12332, t12333, t12334, t12335, t12336, t12337, t12338)
}
