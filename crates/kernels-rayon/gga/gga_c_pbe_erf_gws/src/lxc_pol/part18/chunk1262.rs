//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1262/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1262(t353: f64, t4183: f64, t4386: f64, t810: f64, t14001: f64, t14466: f64, t14765: f64, t3074: f64, t4395: f64, t1161: f64, t874: f64, t3102: f64, t859: f64) -> (f64, f64, f64, f64, f64) {
    let t54550 = t4386 * t353 * t4183 * t810;
    let t54566 = t14001 * t14466;
    let t54567 = 7.0_f64 / 72.0_f64 * t54566;
    let t54580 = t3074 * t4395 * t14765;
    let t54590 = t1161 * t874;
    let t54595 = t859 * t3102;
    (t54550, t54567, t54580, t54590, t54595)
}
