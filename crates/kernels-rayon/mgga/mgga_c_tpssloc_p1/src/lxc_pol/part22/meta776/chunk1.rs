//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2652/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652(t74077: f64, t54411: f64, t120: f64, t20553: f64, t12283: f64, t20454: f64, t20489: f64) -> (f64, f64, f64, f64, f64) {
    let t74078 = 0.5848223622634646207e0_f64 * t74077;
    let t74086 = 3.0_f64 * t54411;
    let t74090 = t120 * t20553;
    let t74110 = t12283 * t20454;
    let t74120 = t120 * t20489;
    (t74078, t74086, t74090, t74110, t74120)
}
