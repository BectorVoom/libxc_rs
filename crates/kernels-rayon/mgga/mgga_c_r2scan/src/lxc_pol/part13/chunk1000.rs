//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1000/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1000(t261: f64, t2720: f64, t3299: f64, t10879: f64, t3594: f64, t2661: f64, t3304: f64, t545: f64, t979: f64, t3300: f64, t2206: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11736 = t261 * t2720;
    let t11737 = t3299 * t11736;
    let t11739 = t10879 * t3594;
    let t11741 = t261 * t2661;
    let t11742 = t3304 * t11741;
    let t11744 = t545 * t979;
    let t11745 = t11744 * t3300;
    let t11747 = t2206 * t978;
    (t11736, t11737, t11739, t11741, t11742, t11744, t11745, t11747)
}
