//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 727/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk727(t296: f64, t3720: f64, t1: f64, t787: f64, t795: f64) -> (f64, f64, f64, f64) {
    let t12250 = t296 * t3720;
    let t12251 = t12250 * t1;
    let t12252 = t787 * t12251;
    let t12255 = t795 * t3720;
    (t12250, t12251, t12252, t12255)
}
