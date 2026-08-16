//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 786/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk786(t13583: f64, t4866: f64, t2621: f64, t9: f64, t7: f64, t118: f64, t4882: f64, t1737: f64, t2471: f64, t1742: f64, t2475: f64, t1734: f64, t2466: f64) -> (f64, f64, f64, f64, f64) {
    let t13584 = t13583 * t4866;
    let t13587 = 1.0_f64 / t9 / t2621;
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    let t13590 = t13589 * t4882;
    let t13592 = t2471 * t1737;
    let t13594 = t2475 * t1742;
    let t13596 = t2466 * t1734;
    (t13584, t13590, t13592, t13594, t13596)
}
