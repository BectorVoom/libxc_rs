//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1321/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1321<F: Float>(t1662: F, t467: F, t11874: F, t1268: F, t1674: F, t1679: F, t1713: F, t1734: F, t20016: F, t20018: F, t20019: F, t20021: F, t20022: F, t20023: F, t2637: F, t3988: F, t5651: F, t6614: F, t694: F, t695: F) -> F {
    let t24605 = t1662 * t467;
    let t24617 = -t1268 * t1679 * t6614 - F::new(6.0) * t1674 * t1713 * t2637 + F::new(12.0) * t1674 * t5651 * t695 + F::new(8.0) * t1679 * t24605 * t3988 - F::new(3.0) * t1734 * t2637 * t694 + t11874 - t20016 + t20018 - t20019 + t20021 - t20022 - t20023;
    t24617
}
