//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 806/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk806(t32215: f64, t5539: f64, t9647: f64, t2563: f64, t3487: f64, t7284: f64, t29277: f64, t32607: f64, t10639: f64, t16879: f64, t883: f64, t10736: f64, t7064: f64) -> (f64, f64, f64, f64, f64) {
    let t42956 = t9647 * t5539 * t32215;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    let t42963 = t9647 * t29277 * t32607;
    let t42967 = t9647 * t16879 * t883 * t10639;
    let t42970 = t7064 * t29277 * t10736;
    (t42956, t42960, t42963, t42967, t42970)
}
