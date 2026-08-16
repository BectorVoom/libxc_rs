//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2288/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288(t100930: f64, t1873: f64, t20162: f64, t6534: f64, t26545: f64, t33185: f64, t12524: f64, t28896: f64, t3941: f64, t5493: f64, t2174: f64, t6470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100932 = 27.0_f64 * t100930 * t1873;
    let t100934 = 0.135e2_f64 * t20162 * t6534;
    let t100936 = 54.0_f64 * t33185 * t26545;
    let t100938 = 54.0_f64 * t12524 * t28896;
    let t100941 = 27.0_f64 * t3941 * t6534 * t5493;
    let t103103 = t6470 * t2174;
    (t100932, t100934, t100936, t100938, t100941, t103103)
}
