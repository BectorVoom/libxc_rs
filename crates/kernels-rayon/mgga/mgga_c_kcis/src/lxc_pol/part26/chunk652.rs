//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 652/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk652(t1517: f64, t4230: f64, t6281: f64, t1518: f64, t6284: f64, t509: f64, t7190: f64, t1153: f64, t1991: f64, t1995: f64, t2018: f64, t368: f64, t4202: f64, t4213: f64, t545: f64, t562: f64, t5966: f64, t5985: f64, t7233: f64, t7237: f64, t7241: f64, t7245: f64, t7249: f64, t7341: f64, t7361: f64, t7365: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t7369 = t1517 * t4230 * t6281;
    let t7373 = t1517 * t1518 * t6284;
    let t7376 = t509 * t7190;
    let t7380 = 0.619125e-2_f64 * t7341 * t545 + 0.1857375e-1_f64 * t2018 * t1991 - 0.123825e-1_f64 * t2018 * t1995 + 0.46434375e-2_f64 * t562 * t7233 - 0.1857375e-1_f64 * t4202 * t7237 + 0.9286875e-2_f64 * t562 * t7241 + 0.123825e-1_f64 * t562 * t7245 - 0.619125e-2_f64 * t562 * t7249 + t4213 - 0.35374814814814814814e-1_f64 * t5966 - 0.53062222222222222222e-1_f64 * t5985 - 0.44218518518518518518e-1_f64 * t1153 * t7361 - 0.53062222222222222222e-1_f64 * t1153 * t7365 + 0.53062222222222222222e-1_f64 * t1153 * t7369 - 0.26531111111111111111e-1_f64 * t1153 * t7373 - 0.39796666666666666666e-1_f64 * t86 * t368 * t7376;
    (t7369, t7373, t7376, t7380)
}
