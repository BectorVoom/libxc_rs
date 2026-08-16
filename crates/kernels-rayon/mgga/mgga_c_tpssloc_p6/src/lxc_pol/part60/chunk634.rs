//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 634/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk634(t1734: f64, t68: f64, t475: f64, t7328: f64, t1730: f64, t2140: f64, t1742: f64, t2139: f64, t471: f64, t1726: f64, t1737: f64, t1748: f64, t2134: f64, t2136: f64, t467: f64, t488: f64, t7309: f64, t7310: f64, t7315: f64, t7326: f64, t7339: f64, t7343: f64, t7345: f64, t8020: f64, t8028: f64, t8031: f64, t8035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8038 = t1734 * t68;
    let t8039 = t8038 * t475;
    let t8040 = t7328 * t8039;
    let t8043 = t1730 * t2140;
    let t8048 = t2139 * t1742;
    let t8049 = t471 * t8048;
    let t8054 = -t8020 * t467 / 36.0_f64 + t7309 - t7310 * t1726 / 288.0_f64 - 0.80745512188280781712e-3_f64 * t8028 * t2136 + t7315 - 0.10093189023535097714e-3_f64 * t8031 * t2136 - 0.10093189023535097714e-3_f64 * t2134 * t8035 + 0.10093189023535097714e-3_f64 * t7326 * t8040 + t8043 * t488 / 1536.0_f64 + t7339 * t1737 / 1536.0_f64 - t8049 * t488 / 288.0_f64 + t7343 - t7345 * t1748 / 2304.0_f64;
    (t8039, t8040, t8043, t8048, t8049, t8054)
}
