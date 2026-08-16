//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1882/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1882(t6562: f64, t6572: f64, t86893: f64, t23171: f64, t23228: f64, t7488: f64, t214: f64, t4265: f64, t1880: f64, t25055: f64, t81591: f64, t25217: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87776 = t6562 * t86893 * t6572;
    let t87779 = t23171 * t23228 * t7488;
    let t87782 = t214 * t4265;
    let t87784 = t1880 * t87782 * t6572;
    let t87786 = t81591 * t25055;
    let t87796 = t6547 * t25217;
    (t87776, t87779, t87782, t87784, t87786, t87796)
}
