//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3423/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3423<F: Float>(t2942: F, t6152: F, t11409: F, t11461: F, t11554: F, t15249: F, t15259: F, t15284: F, t15287: F, t15350: F, t15406: F, t15413: F, t19269: F, t19290: F, t19294: F, t19297: F, t19300: F, t2944: F, t2945: F, t2968: F, t2970: F, t41779: F, t41788: F, t41799: F, t4690: F, t4712: F, t52370: F, t52440: F, t52459: F, t52637: F, t52837: F, t6158: F, t6177: F, t63679: F, t63916: F, t64109: F, t64197: F, t64212: F, t64228: F, t64244: F, t64261: F, t64277: F, t64294: F, t64310: F, t946: F, t954: F, t972: F) -> F {
    let t64319 = t6152 * t2942;
    let t64324 = F::cast_from(0.41016075432865626631e4_f64) * t52370 * t52459 * t972 + F::cast_from(0.64327917994770140268e2_f64) * t2968 * t64109 * t2970 + t63679 - F::cast_from(24.0_f64) * t11409 * t6158 * t2944 + F::cast_from(12.0_f64) * t15406 * t15287 - F::cast_from(0.46785788981077169656e1_f64) * t52440 * t4690 + F::cast_from(0.69263436422725855034e2_f64) * t52637 * t4712 - F::cast_from(0.46785788981077169656e1_f64) * t15413 * t15249 + F::cast_from(0.69263436422725855034e2_f64) * t15350 * t15259 + F::cast_from(0.70178683471615754484e1_f64) * t11461 * t19290 - F::cast_from(0.46785788981077169656e1_f64) * t11554 * t19294 - F::cast_from(0.2077903092681775651e3_f64) * t41788 * t19297 - F::cast_from(0.23392894490538584828e1_f64) * t11554 * t19300 - F::cast_from(0.38596750796862084162e3_f64) * t41779 * t19269 + F::cast_from(1.0_f64) * t946 * (t64197 + t64212 + t64228 + t64244 + t64261 + t64277 + t64294 + t64310) * t954 + F::cast_from(0.32163958997385070134e2_f64) * t41799 * t6177 + t63916 - F::cast_from(2.0_f64) * t64319 * t2945 - F::cast_from(0.77193501593724168323e3_f64) * t52837 * t15284;
    t64324
}
