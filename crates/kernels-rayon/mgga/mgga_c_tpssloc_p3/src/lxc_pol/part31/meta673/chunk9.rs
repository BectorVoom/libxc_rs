//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2033/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2033(t225: f64, t29290: f64, t29293: f64, t1386: f64, t16022: f64, t16460: f64, t20026: f64, t2092: f64, t24082: f64, t26990: f64, t27062: f64, t5215: f64, t56434: f64, t56596: f64, t6461: f64, t7194: f64, t7925: f64, t7937: f64, t97626: f64, t97705: f64) -> f64 {
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102936 = -2.0_f64 * t102917 * t1386 - t56596 * t2092 - t56434 * t2092 - t102922 * t1386 + 4.0_f64 * t16022 * t7925 - 2.0_f64 * t16460 * t7937 + 0.3289868133696452873e-1_f64 * t97705 - t24082 * t6461 + 4.0_f64 * t5215 * t27062 + 2.0_f64 * t7194 * t20026 - 12.0_f64 * t97626 * t26990;
    t102936
}
