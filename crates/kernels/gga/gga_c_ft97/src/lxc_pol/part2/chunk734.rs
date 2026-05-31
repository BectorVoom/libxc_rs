//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 734/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk734<F: Float>(t8360: F, t979: F, t83: F, t1825: F, t3255: F, t11487: F, t11493: F, t11498: F, t11503: F, t11506: F, t11509: F, t11513: F, t11517: F, t11522: F, t11527: F, t11531: F, t11535: F, t11537: F, t1901: F, t446: F) -> (F, F, F) {
    let t11538 = t8360 * t979;
    let t11539 = t83 * t11538;
    let t11542 = t1825 * t3255;
    let t11543 = t83 * t11542;
    let t11546 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11487 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11493 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11498 + t446 * t11503 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t11506 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t11509 - t446 * t11513 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t11517 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t11522 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11527 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11531 + t11535 + t11537 - t446 * t11539 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11543;
    (t11538, t11542, t11546)
}
