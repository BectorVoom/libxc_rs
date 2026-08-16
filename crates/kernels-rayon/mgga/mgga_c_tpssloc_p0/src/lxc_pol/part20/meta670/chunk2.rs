//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2519/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2519(t51078: f64, t51098: f64, t1100: f64, t1107: f64, t51034: f64, t51037: f64, t51040: f64, t51041: f64, t51043: f64, t51046: f64, t51049: f64, t51051: f64, t51053: f64, t51056: f64) -> (f64, f64, f64) {
    let t51099 = t51078 + t51098;
    let t51100 = t1100 * t51099;
    let t51102 = t1107 * t51099;
    let t51104 = 0.11038e0_f64 * t51034 - 0.49671e0_f64 * t51037 + t51040 - 0.33114e0_f64 * t51041 - 0.99342e0_f64 * t51043 - 0.82785e-1_f64 * t51046 - 0.49671e0_f64 * t51049 - 0.91983333333333333334e-1_f64 * t51051 - 0.66228e0_f64 * t51053 + 0.49671e0_f64 * t51056 + 0.258925e1_f64 * t51100 + 0.16504875e0_f64 * t51102;
    (t51100, t51102, t51104)
}
