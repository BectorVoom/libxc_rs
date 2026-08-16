//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 925/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk925(t1058: f64, t5842: f64, t614: f64, t6615: f64, t2178: f64, t6685: f64, t27391: f64, t604: f64, t22511: f64, t32772: f64, t3392: f64, t5818: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104289 = t5842 * t1058;
    let t104364 = t6615 * t614;
    let t104462 = t6685 * t2178;
    let t104623 = t27391 * t604;
    let t104721 = t32772 * t22511;
    let t104722 = t3392 * t104721;
    let t104732 = t5818 * t104721;
    (t104289, t104364, t104462, t104623, t104722, t104732)
}
