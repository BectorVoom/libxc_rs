//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1144/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1144(t7014: f64, t9364: f64, t2365: f64, t23911: f64, t4391: f64, t20065: f64, t544: f64, t9287: f64, t1391: f64, t587: f64, t9547: f64, t20117: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30779 = t7014 * t9364;
    let t30788 = 0.3575048995185042667e0_f64 * t4391 * t2365 * t23911;
    let t30789 = t544 * t20065;
    let t30791 = 0.29792074959875355558e-1_f64 * t30789 * t9287;
    let t30793 = t587 * t1391 * t9547;
    let t30802 = t883 * t20117;
    (t30779, t30788, t30789, t30791, t30793, t30802)
}
