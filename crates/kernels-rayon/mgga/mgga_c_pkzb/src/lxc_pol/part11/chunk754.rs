//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 754/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk754(t6087: f64, t6174: f64, t2295: f64, t877: f64, t2256: f64, t858: f64, t369: f64, t6230: f64) -> (f64, f64, f64, f64, f64) {
    let t6249 = 0.16068111111111111111e1_f64 * t6087;
    let t6256 = 0.46308888888888888888e0_f64 * t6174;
    let t6266 = t877 * t2295;
    let t6272 = t858 * t2256;
    let t6282 = t369 * t6230;
    (t6249, t6256, t6266, t6272, t6282)
}
