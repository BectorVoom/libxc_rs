//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3423/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3423(t2942: f64, t6152: f64, t11409: f64, t11461: f64, t11554: f64, t15249: f64, t15259: f64, t15284: f64, t15287: f64, t15350: f64, t15406: f64, t15413: f64, t19269: f64, t19290: f64, t19294: f64, t19297: f64, t19300: f64, t2944: f64, t2945: f64, t2968: f64, t2970: f64, t41779: f64, t41788: f64, t41799: f64, t4690: f64, t4712: f64, t52370: f64, t52440: f64, t52459: f64, t52637: f64, t52837: f64, t6158: f64, t6177: f64, t63679: f64, t63916: f64, t64109: f64, t64197: f64, t64212: f64, t64228: f64, t64244: f64, t64261: f64, t64277: f64, t64294: f64, t64310: f64, t946: f64, t954: f64, t972: f64) -> f64 {
    let t64319 = t6152 * t2942;
    let t64324 = 0.41016075432865626631e4_f64 * t52370 * t52459 * t972 + 0.64327917994770140268e2_f64 * t2968 * t64109 * t2970 + t63679 - 24.0_f64 * t11409 * t6158 * t2944 + 12.0_f64 * t15406 * t15287 - 0.46785788981077169656e1_f64 * t52440 * t4690 + 0.69263436422725855034e2_f64 * t52637 * t4712 - 0.46785788981077169656e1_f64 * t15413 * t15249 + 0.69263436422725855034e2_f64 * t15350 * t15259 + 0.70178683471615754484e1_f64 * t11461 * t19290 - 0.46785788981077169656e1_f64 * t11554 * t19294 - 0.2077903092681775651e3_f64 * t41788 * t19297 - 0.23392894490538584828e1_f64 * t11554 * t19300 - 0.38596750796862084162e3_f64 * t41779 * t19269 + 1.0_f64 * t946 * (t64197 + t64212 + t64228 + t64244 + t64261 + t64277 + t64294 + t64310) * t954 + 0.32163958997385070134e2_f64 * t41799 * t6177 + t63916 - 2.0_f64 * t64319 * t2945 - 0.77193501593724168323e3_f64 * t52837 * t15284;
    t64324
}
