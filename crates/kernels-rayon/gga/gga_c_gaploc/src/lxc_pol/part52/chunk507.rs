//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 507/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk507(t5345: f64, t5348: f64, t9106: f64, t2519: f64, t3220: f64, t3225: f64, t716: f64, t2524: f64, t871: f64, t1890: f64, t723: f64, t2558: f64, t7634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9672 = t5345 * t9106 * t5348;
    let t9674 = t3220 * t2519;
    let t9676 = t3225 * t716;
    let t9682 = t2524 * t871;
    let t9740 = t1890 * t723;
    let t9752 = t7634 * t2558;
    (t9672, t9674, t9676, t9682, t9740, t9752)
}
