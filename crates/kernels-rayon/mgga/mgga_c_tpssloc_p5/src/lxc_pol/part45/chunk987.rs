//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 987/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk987(t22986: f64, t23270: f64, t31332: f64, t87036: f64, t31338: f64, t82159: f64, t31329: f64, t6547: f64, t1880: f64, t214: f64, t225: f64, t24234: f64, t258: f64) -> (f64, f64, f64, f64) {
    let t114877 = t22986 * t23270 * t31332 * t87036;
    let t114880 = t22986 * t82159 * t31338;
    let t114882 = t6547 * t31329;
    let t114889 = t1880 * t214 * t24234 * t225 * t258;
    (t114877, t114880, t114882, t114889)
}
