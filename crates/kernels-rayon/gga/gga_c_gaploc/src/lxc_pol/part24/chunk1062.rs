//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1062/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1062(t1: f64, t21888: f64, t787: f64, t5654: f64, t7809: f64, t7313: f64, t900: f64, t7173: f64, t2683: f64, t6099: f64, t1964: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22295 = t787 * t21888 * t1;
    let t22315 = t5654 * t7809;
    let t22333 = t900 * t7313;
    let t22405 = t900 * t7173;
    let t22424 = t6099 * t2683;
    let t22537 = t1964 * t9419;
    (t22295, t22315, t22333, t22405, t22424, t22537)
}
