//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 854/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk854(t38907: f64, t739: f64, t12161: f64, t2089: f64, t7290: f64, t321: f64, t3720: f64, t107: f64, t787: f64, t12251: f64, t1980: f64, t296: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39022 = t739 * t38907;
    let t39027 = t2089 * t12161;
    let t39040 = t7290 * t38907;
    let t39048 = t321 * t3720;
    let t39050 = t787 * t39048 * t107;
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    (t39022, t39027, t39040, t39048, t39050, t39118, t39121)
}
