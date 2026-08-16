//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2464/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2464(t5866: f64, t6739: f64, t1022: f64, t14211: f64, t14218: f64, t360: f64, t1615: f64, t883: f64, t1539: f64, t4649: f64, t17906: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70081 = t5866 * t6739;
    let t70082 = t14211 * t1022;
    let t70086 = t14218 * t360;
    let t70100 = t1615 * t883;
    let t70106 = t1539 * t4649;
    let t70122 = t5866 * t1615;
    let t70132 = t4644 * t17906;
    (t70081, t70082, t70086, t70100, t70106, t70122, t70132)
}
