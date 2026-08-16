//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 798/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk798(t12604: f64, t2549: f64, t1902: f64, t883: f64, t7064: f64, t9756: f64, t9624: f64, t9647: f64, t9648: f64, t28648: f64, t5539: f64, t28652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40775 = t2549 * t12604;
    let t40820 = t883 * t1902;
    let t40822 = t7064 * t9756 * t40820;
    let t40825 = t9647 * t9648 * t9624;
    let t40828 = t7064 * t5539 * t28648;
    let t40833 = t9647 * t5539 * t28652;
    (t40775, t40820, t40822, t40825, t40828, t40833)
}
