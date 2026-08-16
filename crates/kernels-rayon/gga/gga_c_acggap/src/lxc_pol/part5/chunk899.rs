//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 899/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk899(t1032: f64, t3531: f64, t3761: f64, t1036: f64, t12254: f64, t175: f64, t398: f64, t1005: f64, t121: f64, t126: f64, t147: f64, t7321: f64) -> (f64, f64, f64, f64, f64) {
    let t13371 = t1032 * t3531;
    let t13373 = t1032 * t3761;
    let t13399 = 0.17149607247227894789e-2_f64 * t1036 * t398 * t175 * t12254;
    let t13400 = t1005 * t3761;
    let t13451 = 455.0_f64 / 243.0_f64 * t121 * t7321 * t126 * t147;
    (t13371, t13373, t13399, t13400, t13451)
}
