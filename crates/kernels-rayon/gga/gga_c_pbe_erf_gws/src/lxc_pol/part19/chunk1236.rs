//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1236/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1236(t3202: f64, t3955: f64, t14113: f64, t14614: f64, t14001: f64, t14463: f64, t3291: f64, t51214: f64, t14063: f64, t8962: f64, t51201: f64, t51222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53970 = t3955 * t3202;
    let t53975 = t14113 * t14614;
    let t53985 = t14001 * t14463;
    let t54014 = t51214 * t3291;
    let t54023 = t14063 * t8962;
    let t54026 = 119.0_f64 / 1728.0_f64 * t51201;
    let t54038 = 35.0_f64 / 216.0_f64 * t51222;
    (t53970, t53975, t53985, t54014, t54023, t54026, t54038)
}
