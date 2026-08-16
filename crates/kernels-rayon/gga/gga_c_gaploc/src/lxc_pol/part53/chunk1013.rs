//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1013/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1013(t12078: f64, t1415: f64, t7030: f64, t47953: f64, t6716: f64, t6717: f64, t13800: f64, t4614: f64, t574: f64, t1445: f64, t38413: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t48208 = t1415 * t12078 * t7030;
    let t48211 = t6716 * t6717 * t47953;
    let t48217 = t574 * t4614 * t13800;
    let t48221 = t574 * t1445 * t38413 * t874;
    (t48208, t48211, t48217, t48221)
}
