//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 913/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk913(t123: f64, t7284: f64, t2563: f64, t9647: f64, t5539: f64, t7292: f64, t286: f64, t708: f64, t9095: f64, t1687: f64, t9099: f64, t5337: f64, t5340: f64, t9106: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9648 = t7284 * t123;
    let t9649 = t9648 * t2563;
    let t9651 = 0.1922631557535556071e-2_f64 * t9647 * t9649;
    let t9652 = t5539 * t7292;
    let t9654 = 0.1281754371690370714e-2_f64 * t9647 * t9652;
    let t9664 = t9095 * t286 * t708;
    let t9666 = t9099 * t1687;
    let t9669 = t9106 * t5337 * t5340;
    (t9648, t9649, t9651, t9652, t9654, t9664, t9666, t9669)
}
