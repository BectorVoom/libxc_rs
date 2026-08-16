//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 398/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk398(t3541: f64, t531: f64, t3545: f64, t3516: f64, t569: f64, t568: f64, t600: f64, t3529: f64, t1565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3570 = t531 * t3541;
    let t3573 = t531 * t3545;
    let t3576 = t569 * t3516;
    let t3577 = t568 * t3576;
    let t3581 = t600 * t3516;
    let t3582 = t568 * t3581;
    let t3585 = t569 * t3529;
    let t3586 = t568 * t3585;
    let t3591 = t1565 * t3516;
    let t3592 = t568 * t3591;
    let t3595 = t600 * t3529;
    let t3596 = t568 * t3595;
    (t3570, t3573, t3576, t3577, t3581, t3582, t3585, t3586, t3591, t3592, t3595, t3596)
}
