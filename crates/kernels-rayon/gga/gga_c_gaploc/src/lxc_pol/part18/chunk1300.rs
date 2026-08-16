//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1300/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1300(t10929: f64, t10932: f64, t2012: f64, t10924: f64, t2009: f64, t6134: f64, t1022: f64, t7275: f64, t2021: f64, t10925: f64, t5724: f64, t1402: f64, t2033: f64, t3473: f64) -> (f64, f64, f64, f64, f64) {
    let t33356 = 0.55213813373645879534e2_f64 * t2012 * t10929 * t10932;
    let t33359 = 0.71500979903700853338e0_f64 * t6134 * t10924 * t2009;
    let t33360 = t7275 * t1022;
    let t33363 = 0.71500979903700853338e0_f64 * t2021 * t33360 * t2009;
    let t33365 = 0.35750489951850426669e0_f64 * t10925 * t5724;
    let t33367 = t2033 * t1402 * t3473;
    (t33356, t33359, t33363, t33365, t33367)
}
