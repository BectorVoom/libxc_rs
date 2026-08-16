//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2015/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2015(t103265: f64, t103267: f64, t106006: f64, t106008: f64, t106010: f64, t106012: f64, t106014: f64, t95666: f64, t98960: f64, t98961: f64, t98962: f64, t98964: f64) -> f64 {
    let t110385 = 0.40656002247428262581e-3_f64 * t106006 + 0.51448821741683684367e-2_f64 * t106008 - 0.32012600194825403606e-1_f64 * t106010 + 0.17149607247227894789e-2_f64 * t106012 + 0.16006300097412701803e-1_f64 * t106014 + t98960 - t98961 - t98962 - 0.60976381323476959249e-3_f64 * t98964 - t103265 - t103267 + t95666;
    t110385
}
