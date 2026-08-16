//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 467/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk467(t2551: f64, t306: f64, t2487: f64, t327: f64, t1601: f64, t1602: f64, t1604: f64, t1606: f64, t1608: f64, t1618: f64, t1621: f64, t2502: f64, t2505: f64, t2514: f64, t2518: f64, t2522: f64, t2525: f64, t311: f64) -> (f64, f64, f64) {
    let t2552 = t2551 * t306;
    let t2555 = t327 * t2487;
    let t2558 = 0.10237773105191754_f64 * t2502 - 0.14975624337724558_f64 * t2514 + 0.037002892246025966_f64 * t2518 - 0.037002892246025966_f64 * t2522 - t1601 - t1602 + t1604 - t1606 + t1608 + 0.14975624337724558_f64 * t2525 - t1618 - t1621 - 0.10237773105191754_f64 * t2505 + t2552 * t311 / 6.0_f64 + t2555 * t311 / 6.0_f64;
    (t2552, t2555, t2558)
}
