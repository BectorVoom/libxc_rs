//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1257/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1257(t21126: f64, t2970: f64, t973: f64, t21569: f64, t3070: f64, t42488: f64, t10231: f64, t21122: f64, t21689: f64, t225: f64, t21669: f64, t21684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70867 = t973 * t2970 * t21126;
    let t70912 = t3070 * t42488 * t21569;
    let t70929 = t973 * t10231 * t21122;
    let t70978 = t21689 * t225;
    let t70980 = t21669 * t225;
    let t70987 = t21684 * t225;
    (t70867, t70912, t70929, t70978, t70980, t70987)
}
