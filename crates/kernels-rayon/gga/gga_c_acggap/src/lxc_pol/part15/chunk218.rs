//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 218/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk218(t756: f64, t199: f64, t27: f64, t13: f64, t218: f64, t219: f64, t663: f64, t666: f64, t669: f64, t673: f64, t675: f64, t678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t757 = 0.35616666666666666666e-1_f64 * t756;
    let t758 = t199 * t27;
    let t759 = 1.0_f64 / t758;
    let t760 = t13 * t759;
    let t761 = t218 * t218;
    let t762 = t761 * t219;
    let t763 = t760 * t762;
    let t764 = 2.0_f64 * t763;
    let t771 = -0.42198333333333333333e0_f64 * t663 + 0.84396666666666666666e0_f64 * t666 + 0.39862222222222222223e0_f64 * t669 + 0.68258333333333333333e-1_f64 * t673 + 0.13651666666666666667e0_f64 * t675 + 0.13692777777777777778e0_f64 * t678;
    (t757, t759, t760, t761, t762, t764, t771)
}
