//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1361/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1361(t10954: f64, t2478: f64, t967: f64, t10953: f64, t2523: f64, t2521: f64, t11035: f64, t7070: f64, t11039: f64, t21366: f64, t8983: f64, t9258: f64) -> (f64, f64, f64, f64, f64) {
    let t29644 = 4.0_f64 * t2478 * t10954 * t967;
    let t29645 = t10953 * t2523;
    let t29648 = 0.32163958997385070134e2_f64 * t2521 * t29645 * t967;
    let t29650 = 0.64327917994770140268e2_f64 * t7070 * t11035;
    let t29652 = 0.1034520258385468006e4_f64 * t21366 * t11039;
    let t29654 = 4.0_f64 * t9258 * t8983;
    (t29644, t29648, t29650, t29652, t29654)
}
