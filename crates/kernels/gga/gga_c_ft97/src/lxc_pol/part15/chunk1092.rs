//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1092/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1092<F: Float>(t12680: F, t144: F, t1901: F, t20874: F, t2222: F, t3439: F, t40766: F, t4431: F, t446: F, t4668: F, t4828: F, t51149: F, t63258: F, t78584: F, t78601: F, t78603: F, t78605: F, t78618: F, t87220: F, t87462: F, t9133: F) -> F {
    let t87805 = -F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t51149 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9133 * t2222 * t4431 * t4668 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t78584 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t3439 * t40766 * t87462 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t87220 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t78601 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t78603 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t78605 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t12680 * t20874 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t63258 * t4828 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t78618;
    t87805
}
