//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1401/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1401(t11153: f64, t3439: f64, t11147: f64, t11545: f64, t3247: f64, t415: f64) -> (f64, f64, f64) {
    let t11759 = t3439 * t11153;
    let t11764 = t11545 * t11147;
    let t11778 = 1.0_f64 / t415 / t3247;
    (t11759, t11764, t11778)
}
