//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 807/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk807(t13150: f64, t2013: f64, t10007: f64, t2925: f64, t825: f64, t9438: f64, t3039: f64, t5774: f64, t24549: f64, t7584: f64, t13072: f64, t32757: f64) -> (f64, f64, f64, f64, f64) {
    let t44084 = t2013 * t13150;
    let t44088 = t825 * t9438 * t10007 * t2925;
    let t44090 = t3039 * t5774;
    let t44117 = t7584 * t9438 * t24549;
    let t44130 = t32757 * t13072;
    (t44084, t44088, t44090, t44117, t44130)
}
