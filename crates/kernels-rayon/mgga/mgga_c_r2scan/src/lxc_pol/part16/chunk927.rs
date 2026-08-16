//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 927/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk927(t10710: f64, t6476: f64, t10728: f64, t3344: f64, t776: f64, t2096: f64, t269: f64, t23: f64, t39: f64, t6077: f64, t255: f64, t6321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10729 = t10710 * t6476;
    let t10730 = t10728 * t10729;
    let t10732 = t776 * t3344;
    let t10734 = t2096 * t269;
    let t10737 = 1.0_f64 / t23 / t6077 / t39;
    let t10740 = t10734 * t10737 * t255 * t6321;
    (t10729, t10730, t10732, t10734, t10737, t10740)
}
