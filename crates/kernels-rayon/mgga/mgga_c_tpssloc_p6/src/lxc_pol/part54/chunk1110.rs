//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1110/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1110(t4993: f64, t7345: f64, t5040: f64, t7310: f64, t27607: f64, t460: f64, t24682: f64, t24658: f64, t3: f64, t24719: f64, t3030: f64, t1734: f64, t3503: f64) -> (f64, f64, f64, f64, f64) {
    let t27622 = t7345 * t4993;
    let t27626 = t7310 * t5040;
    let t27628 = t27607 * t460;
    let t27629 = t24682 * t27628;
    let t27634 = t24658 * t3;
    let t27635 = t24719 * t3030;
    let t27636 = t27634 * t27635;
    let t27637 = t3503 * t1734;
    (t27622, t27626, t27629, t27636, t27637)
}
