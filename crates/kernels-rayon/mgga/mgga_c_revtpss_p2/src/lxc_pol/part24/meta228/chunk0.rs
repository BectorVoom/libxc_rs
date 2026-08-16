//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 985/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk985(t1263: f64, t675: f64, t126: f64, t3617: f64, t2434: f64, t371: f64, t482: f64, t481: f64, t1284: f64, t3566: f64, t3624: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12879 = t675 * t1263;
    let t12884 = t126 * t3617;
    let t12898 = t371 * t2434 * t482;
    let t12900 = 0.63517063878621832551e-4_f64 * t481 * t12898;
    let t12909 = t3566 * t1284;
    let t12910 = t12909 * t3624;
    let t12915 = t126 * t482;
    let t12916 = t828 * t12915;
    (t12879, t12884, t12898, t12900, t12909, t12910, t12915, t12916)
}
