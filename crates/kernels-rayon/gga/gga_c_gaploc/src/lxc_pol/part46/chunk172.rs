//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 172/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk172(t723: f64, t808: f64, t568: f64, t325: f64, t579: f64, t61: f64, t120: f64, t320: f64) -> (f64, f64, f64) {
    let t814 = t808 * t723;
    let t815 = t568 * t814;
    let t818 = t579 * t325;
    let t819 = t61 * t818;
    let t822 = t320 * t120;
    (t815, t819, t822)
}
