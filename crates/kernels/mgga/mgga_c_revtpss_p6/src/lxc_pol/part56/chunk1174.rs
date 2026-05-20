//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1174/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1174<F: Float>(t124533: F, t125344: F, t129478: F, t129479: F, t129480: F, t129481: F, t129482: F, t129483: F, t129488: F, t131234: F, t131338: F, t1518: F, t33346: F, t33644: F, t33646: F, t4292: F, t670: F) -> F {
    let t131384 = F::new(2.0) * t124533 * t1518 + F::new(2.0) * t131234 * t670 + F::new(2.0) * t131338 * t1518 + F::new(2.0) * t33346 * t4292 + t125344 + F::new(4.0) * t129478 + F::new(4.0) * t129479 + F::new(4.0) * t129480 + F::new(4.0) * t129481 + F::new(4.0) * t129482 + F::new(4.0) * t129483 + F::new(4.0) * t129488 + t33644 + t33646;
    t131384
}
