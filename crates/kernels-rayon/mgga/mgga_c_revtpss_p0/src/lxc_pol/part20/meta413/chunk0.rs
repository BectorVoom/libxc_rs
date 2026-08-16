//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1525/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1525(t11267: f64, t3123: f64, t3147: f64, t3229: f64, t3141: f64, t3144: f64, t1036: f64, t11922: f64, t12016: f64, t3115: f64, t11638: f64, t3127: f64, t3172: f64) -> (f64, f64, f64, f64, f64) {
    let t42934 = t3123 * t11267;
    let t42937 = t3229 * t3147;
    let t42939 = t3141 * t3144 * t42937;
    let t42943 = t3141 * t1036 * t42937;
    let t42947 = t3115 * t11922 * t12016;
    let t42962 = t3127 * t3172 * t11638;
    (t42934, t42939, t42943, t42947, t42962)
}
