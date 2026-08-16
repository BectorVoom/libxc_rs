//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 734/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk734(t8360: f64, t979: f64, t83: f64, t1825: f64, t3255: f64, t11487: f64, t11493: f64, t11498: f64, t11503: f64, t11506: f64, t11509: f64, t11513: f64, t11517: f64, t11522: f64, t11527: f64, t11531: f64, t11535: f64, t11537: f64, t1901: f64, t446: f64) -> (f64, f64, f64) {
    let t11538 = t8360 * t979;
    let t11539 = t83 * t11538;
    let t11542 = t1825 * t3255;
    let t11543 = t83 * t11542;
    let t11546 = 2.0_f64 / 3.0_f64 * t446 * t11487 - 4.0_f64 / 3.0_f64 * t1901 * t11493 + 2.0_f64 / 3.0_f64 * t446 * t11498 + t446 * t11503 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t11506 + 4.0_f64 / 3.0_f64 * t446 * t11509 - t446 * t11513 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t11517 + 4.0_f64 / 3.0_f64 * t446 * t11522 + 2.0_f64 / 3.0_f64 * t446 * t11527 + 2.0_f64 / 3.0_f64 * t446 * t11531 + t11535 + t11537 - t446 * t11539 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t11543;
    (t11538, t11542, t11546)
}
