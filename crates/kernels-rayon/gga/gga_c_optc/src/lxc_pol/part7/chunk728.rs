//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 728/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk728(t111: f64, t6944: f64, t5: f64, t629: f64, t6856: f64, t2007: f64, t2017: f64, t6560: f64, t2003: f64, t627: f64, t631: f64, t135: f64, t2011: f64, t628: f64, t636: f64, t6901: f64, t6904: f64, t6907: f64, t6910: f64, t6913: f64, t6919: f64, t6925: f64, t6928: f64, t6933: f64, t6938: f64, t6942: f64) -> (f64, f64, f64, f64, f64) {
    let t6945 = t111 * t6944;
    let t6947 = t629 * t5 * t6856;
    let t6950 = t2007 * t2017;
    let t6953 = t629 * t5 * t6560;
    let t6956 = t2003 * t627;
    let t6957 = t6956 * t631;
    let t6959 = 0.15213032607130326246e0_f64 * t6901 - 0.10866451862235947318e-1_f64 * t135 * t6904 - 0.86207184773738515394e0_f64 * t6907 + 3.0_f64 / 16.0_f64 * t2011 * t6910 - 0.76065163035651631229e0_f64 * t6913 - 0.32599355586707841954e0_f64 * t135 * t6919 - t6925 + 0.32599355586707841954e-1_f64 * t636 * t6928 - 0.16299677793353920977e0_f64 * t636 * t6933 + 0.32599355586707841954e-1_f64 * t636 * t6938 - 7.0_f64 / 16.0_f64 * t6942 - t6945 * t6947 / 4.0_f64 + 7.0_f64 / 48.0_f64 * t6950 - t628 * t6953 / 48.0_f64 - 35.0_f64 / 72.0_f64 * t6957;
    (t6945, t6947, t6953, t6956, t6959)
}
