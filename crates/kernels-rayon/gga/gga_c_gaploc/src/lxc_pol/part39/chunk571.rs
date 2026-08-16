//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 571/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk571(t2101: f64, t3209: f64, t1890: f64, t723: f64, t550: f64, t9603: f64, t5539: f64, t9595: f64, t1843: f64, t2558: f64, t7634: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9739 = t2101 * t3209;
    let t9740 = t1890 * t723;
    let t9741 = t9739 * t9740;
    let t9744 = t550 * t9603;
    let t9745 = t5539 * t9744;
    let t9748 = t550 * t9595;
    let t9749 = t1843 * t9748;
    let t9752 = t7634 * t2558;
    let t9754 = 0.64087718584518535698e-3_f64 * t9647 * t9752;
    (t9739, t9740, t9741, t9744, t9745, t9748, t9749, t9754)
}
