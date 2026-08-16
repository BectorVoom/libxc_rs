//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 844/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk844(t13556: f64, t7137: f64, t11595: f64, t1897: f64, t7671: f64, t13489: f64, t2549: f64, t11608: f64, t2580: f64, t7068: f64, t2562: f64, t35558: f64, t883: f64, t943: f64) -> (f64, f64, f64, f64, f64) {
    let t45000 = 0.20508069947045931423e-1_f64 * t7137 * t13556;
    let t45009 = 0.23071578690426672851e-1_f64 * t1897 * t11595 * t7671;
    let t45010 = t2549 * t13489;
    let t45015 = 0.15381052460284448567e-1_f64 * t1897 * t2580 * t11608 * t7068;
    let t45028 = t943 * t2562 * t883 * t35558;
    (t45000, t45009, t45010, t45015, t45028)
}
