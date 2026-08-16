//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 853/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk853(t35678: f64, t762: f64, t242: f64, t241: f64, t258: f64, t35546: f64, t1131: f64, t729: f64, t7560: f64, t193: f64, t33707: f64, t33747: f64, t33765: f64, t35636: f64, t35641: f64, t35645: f64, t35649: f64, t35653: f64, t35657: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t35679 = t762 * t35678;
    let t35680 = t242 * t35679;
    let t35684 = t241 * t35546 * t258;
    let t35689 = t729 * t7560 * t1131;
    let t35692 = -t33707 + t446 * t35636 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t35641 + 2.0_f64 / 3.0_f64 * t446 * t35645 + 4.0_f64 / 3.0_f64 * t446 * t35649 + 4.0_f64 / 3.0_f64 * t446 * t35653 - 2.0_f64 * t446 * t35657 + t33747 - t33765 - t446 * t35680 / 3.0_f64 + t89 * t193 * t35684 / 3.0_f64 - t446 * t35689 / 3.0_f64;
    (t35679, t35680, t35684, t35689, t35692)
}
