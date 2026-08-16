//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1025/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1025(t115238: f64, t115245: f64, t115249: f64, t115251: f64, t115254: f64, t115256: f64, t115261: f64, t115265: f64, t115271: f64, t2039: f64, t2165: f64, t23917: f64, t23953: f64, t23958: f64, t24176: f64, t24442: f64, t24924: f64, t31832: f64, t32365: f64, t4034: f64, t652: f64, t7056: f64, t7171: f64, t7266: f64, t7408: f64, t8690: f64) -> f64 {
    let t117590 = -2.0_f64 * t2039 * t24924 * t652 - 2.0_f64 * t2165 * t23917 * t652 - 4.0_f64 * t652 * t7056 * t7408 + 3.0_f64 * t23953 * t8690 + 6.0_f64 * t23958 * t8690 + 6.0_f64 * t24176 * t8690 - 2.0_f64 * t24442 * t7266 + 6.0_f64 * t31832 * t7171 - 4.0_f64 * t32365 * t4034 + t115238 + t115245 - t115249 - t115251 - t115254 - t115256 - t115261 + t115265 - t115271;
    t117590
}
