//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 467/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk467<F: Float>(t2551: F, t306: F, t2487: F, t327: F, t1601: F, t1602: F, t1604: F, t1606: F, t1608: F, t1618: F, t1621: F, t2502: F, t2505: F, t2514: F, t2518: F, t2522: F, t2525: F, t311: F) -> (F, F, F) {
    let t2552 = t2551 * t306;
    let t2555 = t327 * t2487;
    let t2558 = F::cast_from(0.10237773105191754_f64) * t2502 - F::cast_from(0.14975624337724558_f64) * t2514 + F::cast_from(0.037002892246025966_f64) * t2518 - F::cast_from(0.037002892246025966_f64) * t2522 - t1601 - t1602 + t1604 - t1606 + t1608 + F::cast_from(0.14975624337724558_f64) * t2525 - t1618 - t1621 - F::cast_from(0.10237773105191754_f64) * t2505 + t2552 * t311 / F::new(6.0) + t2555 * t311 / F::new(6.0);
    (t2552, t2555, t2558)
}
