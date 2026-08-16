//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1072/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1072(t261: f64, t3304: f64, t6503: f64, t10872: f64, t10885: f64, t1582: f64, t2096: f64, t571: f64, t10769: f64, t3281: f64, t6245: f64, t120: f64, t6511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37851 = t3304 * t261 * t6503;
    let t37859 = t10872 * t10885;
    let t37880 = t571 * t1582 * t2096;
    let t37881 = t37880 * t10769;
    let t37883 = t3281 * t6245;
    let t37890 = t120 * t6511;
    (t37851, t37859, t37880, t37881, t37883, t37890)
}
