//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1098/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098(t41: f64, t42: f64, t53: f64, t54: f64, t1028: f64, t36: f64, t9576: f64, t2244: f64, t2250: f64, t2262: f64, t2267: f64, t2268: f64, t2271: f64, t2274: f64, t39: f64, t39097: f64, t39103: f64, t39110: f64, t43: f64, t44: f64, t51: f64, t55: f64, t615: f64, t618: f64, t9258: f64, t9277: f64, t9287: f64, t9289: f64, t9292: f64, t9293: f64, t9296: f64, t9300: f64, t9304: f64, sigma0: f64) -> (f64, f64) {
    let t39157 = t41 * t41;
    let t39159 = 1.0_f64 / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = 1.0_f64 / t54 / t39166;
    let t39176 = 1.0_f64 / t36 / t1028;
    let t39177 = sigma0 * t39176;
    let t39210 = 20944.0_f64 / 81.0_f64 * t9576;
    let t39213 = 5.0_f64 / 162.0_f64 * t39 * t39159 * t39097 + 5.0_f64 / 6.0_f64 * t39 * t43 * t39110 + 5.0_f64 / 162.0_f64 * t51 * t39168 * t39097 - 5.0_f64 / 6.0_f64 * t51 * t55 * t39110 + 20944.0_f64 / 81.0_f64 * t39177 * t44 - 12320.0_f64 / 81.0_f64 * t9277 * t618 + 440.0_f64 / 9.0_f64 * t2262 * t2271 + 440.0_f64 / 27.0_f64 * t2262 * t2268 + 40.0_f64 / 81.0_f64 * t615 * t9289 - 80.0_f64 / 9.0_f64 * t615 * t9296 - 5.0_f64 / 18.0_f64 * t39 * t9287 * t2244 * t2250 + 5.0_f64 / 6.0_f64 * t39 * t2267 * t39103 + 10.0_f64 / 9.0_f64 * t39 * t9292 * t9258 + 5.0_f64 / 18.0_f64 * t51 * t9300 * t2244 * t2250 + 5.0_f64 / 6.0_f64 * t51 * t2274 * t39103 + 10.0_f64 / 9.0_f64 * t51 * t9304 * t9258 - t39210 - 80.0_f64 / 9.0_f64 * t615 * t9293;
    (t39177, t39213)
}
