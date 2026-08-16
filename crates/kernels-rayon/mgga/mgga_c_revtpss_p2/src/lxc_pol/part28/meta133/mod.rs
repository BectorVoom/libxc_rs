//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta133(t2944: f64, t954: f64, t2846: f64, t2904: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64) -> (f64, f64, f64, f64) {
        let (t2945, t2950, t2957, t2962) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk742(t2944, t954, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
    (t2945, t2950, t2957, t2962)
}
