//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 274/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk274(t1: f64, t2101: f64, t313: f64) -> (f64, f64) {
    let t2102 = t2101 * t1;
    let t2103 = t313 * t2102;
    (t2102, t2103)
}
