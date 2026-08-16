//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 923/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk923(t10024: f64, t11823: f64, t43881: f64, t44707: f64, t5241: f64, t5640: f64, t590: f64, t11622: f64, t2464: f64, t2465: f64, t825: f64, t13641: f64, t2013: f64) -> (f64, f64, f64, f64, f64) {
    let t45678 = t11823 * t10024;
    let t45680 = 0.15337170381568299871e1_f64 * t43881;
    let t45684 = 0.13803453343411469884e2_f64 * t5640 * t5241 * t44707 * t590;
    let t45687 = t825 * t2464 * t2465 * t11622;
    let t45688 = 0.42603251059911944084e-1_f64 * t45687;
    let t45689 = t2013 * t13641;
    (t45678, t45680, t45684, t45688, t45689)
}
