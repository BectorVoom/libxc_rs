//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1235/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1235(t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64, t1098: f64, t3256: f64, t1094: f64, t3312: f64, t3311: f64, t419: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11145 = t154 * t3584;
    let t11147 = 1.0_f64 / t3241 / t636;
    let t11152 = t3241 * t52;
    let t11153 = 1.0_f64 / t11152;
    let t11180 = t3256 * t1098;
    let t11185 = t1094 * t3312;
    let t11189 = 1.0_f64 / t3311 / t419;
    let t11190 = t409 * t11189;
    (t11145, t11147, t11153, t11180, t11185, t11190)
}
