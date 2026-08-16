//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 639/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk639(t2319: f64, t89: f64, t1266: f64, t671: f64, t107: f64, t2281: f64, t626: f64, t667: f64, t106: f64, t655: f64) -> (f64, f64, f64, f64, f64) {
    let t2320 = t89 * t2319;
    let t2323 = t1266 * t671;
    let t2327 = 11.0_f64 / 9.0_f64 * t2281 * t107;
    let t2328 = t626 * t667;
    let t2331 = 1.0_f64 / t655 / t106;
    (t2320, t2323, t2327, t2328, t2331)
}
