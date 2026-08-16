//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1345/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1345(t324: f64, t76602: f64, t76630: f64, t300: f64, t1589: f64, t69012: f64, t5774: f64) -> (f64, f64, f64, f64) {
    let t76632 = (t76602 + t76630) * t324;
    let t76634 = 0.19751673498613801407e-1_f64 * t300 * t76632;
    let t76636 = 0.23392894490538584828e1_f64 * t69012 * t1589;
    let t76637 = t5774 * t5774;
    (t76632, t76634, t76636, t76637)
}
