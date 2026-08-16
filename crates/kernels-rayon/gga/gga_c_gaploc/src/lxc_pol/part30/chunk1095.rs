//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1095/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1095(t28409: f64, t2021: f64, t6109: f64, t899: f64, t7305: f64, t913: f64, t2033: f64, t2365: f64, t2610: f64, t7112: f64, t15349: f64, t3281: f64) -> (f64, f64, f64, f64, f64) {
    let t28410 = 0.30674340763136599741e1_f64 * t28409;
    let t28412 = t2021 * t6109 * t899;
    let t28415 = 0.11916829983950142223e0_f64 * t28412 * t913 * t7305;
    let t28419 = 0.29792074959875355558e-1_f64 * t2033 * t2365 * t2610 * t7112;
    let t28421 = 0.29792074959875355558e-1_f64 * t15349 * t3281;
    (t28410, t28412, t28415, t28419, t28421)
}
