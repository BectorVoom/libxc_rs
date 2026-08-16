//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1454/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454(t2439: f64, t2440: f64, t6072: f64, t15003: f64, t51258: f64, t6042: f64, t786: f64, t867: f64, t14485: f64, t14987: f64, t2435: f64, t6093: f64) -> (f64, f64, f64, f64, f64) {
    let t63050 = t2439 * t2440 * t6072;
    let t63058 = t51258 * t15003;
    let t63084 = t786 * t6042 * t867;
    let t63099 = t14987 * t14485;
    let t63453 = t2435 * t6093;
    (t63050, t63058, t63084, t63099, t63453)
}
