//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 903/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk903(t39695: f64, t6520: f64, t6525: f64, t7888: f64, t2326: f64, t3394: f64, t6514: f64, t9074: f64, t30204: f64, t31769: f64, t10177: f64, t19531: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t42638 = 0.63233348079280332443e-2_f64 * t39695;
    let t42640 = t6525 * t7888 * t6520;
    let t42641 = 0.71137516589190373998e-2_f64 * t42640;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42645 = 0.16598753870811087267e-1_f64 * t42644;
    let t42647 = t9074 * t30204 * t31769;
    let t42648 = 0.284550066356761496e-1_f64 * t42647;
    let t42651 = t9074 * t19531 * t883 * t10177;
    (t42638, t42641, t42645, t42648, t42651)
}
