//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 882/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk882(t12623: f64, t2549: f64, t10053: f64, t2558: f64, t943: f64, t12604: f64, t1902: f64, t883: f64, t7064: f64, t9756: f64, t9624: f64, t9647: f64, t9648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40752 = t2549 * t12623;
    let t40758 = t943 * t10053 * t2558;
    let t40775 = t2549 * t12604;
    let t40820 = t883 * t1902;
    let t40822 = t7064 * t9756 * t40820;
    let t40825 = t9647 * t9648 * t9624;
    (t40752, t40758, t40775, t40820, t40822, t40825)
}
