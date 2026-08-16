//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 366/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk366(t1457: f64, t2950: f64, t1035: f64, t769: f64, t2925: f64, t314: f64, t313: f64, t2963: f64, t531: f64, t808: f64, t568: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3043 = t1457 * t2950;
    let t3046 = t769 * t1035;
    let t3049 = t314 * t2925;
    let t3050 = t313 * t3049;
    let t3055 = t531 * t2963;
    let t3060 = t808 * t2925;
    let t3061 = t568 * t3060;
    let t3066 = t836 * t2925;
    (t3043, t3046, t3049, t3050, t3055, t3061, t3066)
}
