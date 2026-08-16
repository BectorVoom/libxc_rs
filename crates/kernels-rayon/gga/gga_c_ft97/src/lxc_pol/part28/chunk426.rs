//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 426/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk426(t488: f64, t6557: f64, t83: f64, t1901: f64, t28: f64, t446: f64, t5629: f64, t5655: f64, t5716: f64, t6466: f64, t6471: f64, t6475: f64, t6480: f64, t6484: f64, t6488: f64, t6492: f64, t6526: f64, t6531: f64, t6535: f64, t6540: f64, t6544: f64, t6549: f64, t89: f64) -> (f64, f64) {
    let t6558 = t488 * t6557;
    let t6559 = t83 * t6558;
    let t6562 = t5629 + t1901 * t6466 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6471 - t446 * t6475 / 3.0_f64 + t446 * t6480 / 3.0_f64 - t446 * t6484 / 3.0_f64 - t5655 - t446 * t6488 / 9.0_f64 - t446 * t6492 / 3.0_f64 + t89 * t28 * t6526 / 3.0_f64 - t446 * t6531 / 3.0_f64 + t5716 + t1901 * t6535 / 9.0_f64 + t446 * t6540 / 3.0_f64 - t446 * t6544 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6549 - t446 * t6559 / 3.0_f64;
    (t6559, t6562)
}
