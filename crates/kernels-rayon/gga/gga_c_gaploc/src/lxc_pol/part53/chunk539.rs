//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 539/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk539(t286: f64, t708: f64, t9095: f64, t1687: f64, t9099: f64, t5337: f64, t5340: f64, t9106: f64, t5345: f64, t5348: f64, t2519: f64, t3220: f64) -> (f64, f64, f64, f64, f64) {
    let t9664 = t9095 * t286 * t708;
    let t9666 = t9099 * t1687;
    let t9669 = t9106 * t5337 * t5340;
    let t9672 = t5345 * t9106 * t5348;
    let t9674 = t3220 * t2519;
    (t9664, t9666, t9669, t9672, t9674)
}
