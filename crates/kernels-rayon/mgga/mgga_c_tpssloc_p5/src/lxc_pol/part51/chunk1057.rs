//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1057/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1057(t1268: f64, t26135: f64, t1458: f64, t22461: f64, t24999: f64, t26098: f64, t26103: f64, t26109: f64, t26111: f64, t26113: f64, t26116: f64, t26119: f64, t26121: f64, t26123: f64, t26125: f64, t4072: f64, t6517: f64, t671: f64) -> f64 {
    let t26137 = 2.0_f64 * t1268 * t26135;
    let t26138 = 2.0_f64 * t1458 * t22461 + 2.0_f64 * t1458 * t26103 + 2.0_f64 * t24999 * t671 + 2.0_f64 * t4072 * t6517 + t26098 + t26109 + t26111 + t26113 + t26116 + t26119 + t26121 + t26123 + t26125 + t26137;
    t26138
}
