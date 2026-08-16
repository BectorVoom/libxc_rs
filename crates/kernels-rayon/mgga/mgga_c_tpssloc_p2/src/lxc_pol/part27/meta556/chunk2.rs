//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1999/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1999(t3701: f64, t6995: f64, t1862: f64, t31: f64, t607: f64, t7752: f64, t1390: f64, t22811: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31035 = t3701 * t6995;
    let t31682 = t1862 * t31;
    let t31683 = t31682 * t607;
    let t33136 = t3701 * t7752;
    let t34475 = t6995 * t1390;
    let t39041 = 1.0_f64 / t22811;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    (t31035, t31683, t33136, t34475, t39041, t39049, t39054)
}
