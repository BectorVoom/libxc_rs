//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1066/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1066(t24230: f64, t24231: f64, t25109: f64, t25126: f64, t25133: f64, t25140: f64, t25144: f64, t28380: f64, t28384: f64, t28386: f64, t28390: f64, t28397: f64, t28399: f64, t28401: f64, t28403: f64) -> f64 {
    let t29039 = 0.33913115119077928316e-1_f64 * t25109 + t28380 / 96.0_f64 - 0.24223653656484234512e-2_f64 * t28384 + t28386 / 8.0_f64 + 0.16956557559538964158e-1_f64 * t28390 + 0.56521858531796547194e-2_f64 * t25126 + 0.13457585364713463618e-3_f64 * t25133 + 0.48447307312968469024e-2_f64 * t28397 + 7.0_f64 / 36.0_f64 * t25140 - t28399 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t28401 + 7.0_f64 / 576.0_f64 * t25144 - t28403 / 24.0_f64 + t24230 + t24231;
    t29039
}
