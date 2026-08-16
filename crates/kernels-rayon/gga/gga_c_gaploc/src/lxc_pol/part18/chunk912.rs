//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 912/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk912(t5345: f64, t5348: f64, t9106: f64, t2519: f64, t3220: f64, t3225: f64, t716: f64, t2558: f64, t7634: f64, t9647: f64, t1843: f64, t7069: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9672 = t5345 * t9106 * t5348;
    let t9674 = t3220 * t2519;
    let t9676 = t3225 * t716;
    let t9752 = t7634 * t2558;
    let t9754 = 0.64087718584518535698e-3_f64 * t9647 * t9752;
    let t9760 = t1843 * t7069;
    (t9672, t9674, t9676, t9752, t9754, t9760)
}
