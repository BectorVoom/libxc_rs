//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1213/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213(t13965: f64, t4641: f64, t1020: f64, t10508: f64, t248: f64, t5867: f64, t3039: f64, t5878: f64, t14202: f64, t4644: f64, t3082: f64, t5905: f64) -> (f64, f64, f64, f64, f64) {
    let t62148 = t4641 * t13965;
    let t62177 = t1020 * t248 * t10508 * t5867;
    let t62183 = t3039 * t248 * t10508 * t5878;
    let t62284 = t4644 * t14202;
    let t62360 = t5905 * t3082;
    (t62148, t62177, t62183, t62284, t62360)
}
