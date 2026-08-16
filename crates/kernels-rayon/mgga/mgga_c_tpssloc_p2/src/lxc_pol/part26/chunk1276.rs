//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1276/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1276(t2587: f64, t81151: f64, t23172: f64, t23150: f64, t814: f64, t25084: f64, t9634: f64, t23097: f64, t2628: f64, t2632: f64, t47320: f64, t46519: f64, t6605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = 0.98696044010893586188e-1_f64 * t81716;
    let t81718 = t814 * t23150;
    let t81724 = t25084 * t9634;
    let t81728 = t23097 * t2628 * t47320 * t2632;
    let t81731 = t6605 * t2628 * t46519;
    (t81715, t81717, t81718, t81724, t81728, t81731)
}
