//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 634/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk634(t2835: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t408: f64, t1019: f64, t1023: f64, t1044: f64, t1022: f64, t404: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2854 = t2835 - 0.11872222222222222222e-1_f64 * t2836 - 0.11872222222222222222e-1_f64 * t2843 + 0.35616666666666666666e-1_f64 * t2848 + 0.17808333333333333333e-1_f64 * t2852;
    let t2856 = 0.621814e-1_f64 * t2854 * t408;
    let t2857 = t1019 * t1023;
    let t2859 = 2.0_f64 * t2857 * t1044;
    let t2860 = t1022 * t404;
    let t2861 = 1.0_f64 / t2860;
    let t2862 = t394 * t2861;
    (t2854, t2856, t2857, t2859, t2861, t2862)
}
