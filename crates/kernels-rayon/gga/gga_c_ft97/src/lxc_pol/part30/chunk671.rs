//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 671/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk671(t1234: f64, t856: f64, t6318: f64, t840: f64, t28735: f64, t824: f64, t2862: f64, t24980: f64, t2: f64, t7021: f64, t2665: f64, t684: f64) -> (f64, f64, f64, f64, f64) {
    let t28736 = t1234 * t856;
    let t28738 = t840 * t6318 * t28736;
    let t28739 = t28735 * t28738;
    let t28741 = t1234 * t824;
    let t28743 = t2862 * t6318 * t28741;
    let t28744 = t24980 * t28743;
    let t28746 = t2 * t7021;
    let t28748 = t2665 * t28746 * t684;
    (t28736, t28739, t28741, t28744, t28748)
}
