//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1464/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1464(t120912: f64, t120924: f64, t120926: f64, t120928: f64, t120930: f64, t120940: f64, t120941: f64, t123198: f64, t2314: f64, t26875: f64, t26902: f64, t31832: f64, t32350: f64, t34150: f64, t4034: f64, t4073: f64, t652: f64, t7408: f64, t7801: f64, t7941: f64, t8690: f64) -> f64 {
    let t124890 = -2.0_f64 * t652 * t7408 * t7801 + 6.0_f64 * t123198 * t26875 - 2.0_f64 * t2314 * t34150 - t26902 * t8690 + t31832 * t7941 - 2.0_f64 * t32350 * t4073 - 2.0_f64 * t34150 * t4034 - t120912 - t120924 - t120926 - t120928 - t120930 + t120940 - t120941;
    t124890
}
