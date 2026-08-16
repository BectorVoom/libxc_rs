//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 345/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk345(t524: f64, t999: f64, t189: f64, t2754: f64, t188: f64, t2792: f64, t531: f64, t569: f64, t568: f64, t1457: f64, t2779: f64, t2778: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2819 = t524 * t999;
    let t2822 = t189 * t2754;
    let t2823 = t188 * t2822;
    let t2828 = t531 * t2792;
    let t2833 = t569 * t2754;
    let t2834 = t568 * t2833;
    let t2843 = t1457 * t2779;
    let t2846 = t2778 * t475;
    (t2819, t2822, t2823, t2828, t2834, t2843, t2846)
}
