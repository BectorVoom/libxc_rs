//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3579/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3579(t43771: f64, t43781: f64, t43783: f64, t43814: f64, t43817: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64) -> f64 {
    let t68284 = 0.12077e1_f64 * t68253 + 0.13418888888888888889e0_f64 * t68255 - 0.89459259259259259257e-1_f64 * t68257 - 0.49057777777777777778e0_f64 * t43771 + 0.91983333333333333333e-1_f64 * t43781 + 0.18396666666666666667e0_f64 * t43783 + t43814 + t43817 - 0.22364814814814814814e0_f64 * t68262 + 0.33547222222222222222e0_f64 * t68267 + 0.72462e1_f64 * t68271 + 0.12077e1_f64 * t68275 - 0.40256666666666666667e0_f64 * t68277 - 0.40256666666666666666e0_f64 * t68282;
    t68284
}
