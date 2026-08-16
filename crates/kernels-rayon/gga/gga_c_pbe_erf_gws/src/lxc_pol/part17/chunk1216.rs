//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1216/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1216(t321: f64, t50825: f64, t1167: f64, t2423: f64, t3324: f64, t810: f64, t1172: f64, t1198: f64, t319: f64, t13763: f64, t8546: f64, t2494: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52061 = t321 * t50825;
    let t52763 = t1167 * t2423;
    let t52767 = t3324 * t810;
    let t52774 = t1172 * t319 * t1198;
    let t52775 = t8546 * t13763;
    let t52782 = t2494 * t944;
    (t52061, t52763, t52767, t52774, t52775, t52782)
}
