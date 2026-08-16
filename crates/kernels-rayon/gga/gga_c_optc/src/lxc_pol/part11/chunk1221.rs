//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1221/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1221(t1271: f64, t48590: f64, t162: f64, t1256: f64, t48308: f64, t2034: f64, t13248: f64, t4595: f64, t1277: f64, t16401: f64, t16405: f64, t2011: f64, t2021: f64, t22158: f64, t38553: f64, t48212: f64, t48214: f64, t5: f64, t55893: f64, t55933: f64, t56074: f64, t56078: f64, t56082: f64, t56103: f64, t56107: f64, t628: f64, t629: f64, t636: f64, t9601: f64, t9642: f64, t9678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56110 = t48590 * t1271;
    let t56111 = t162 * t56110;
    let t56114 = t48308 * t1256;
    let t56115 = t2034 * t56114;
    let t56118 = t13248 * t4595;
    let t56119 = t2034 * t56118;
    let t56122 = 0.43465807448943789272e-1_f64 * t636 * t56074 - 0.32599355586707841954e0_f64 * t636 * t56078 - 0.13039742234683136782e0_f64 * t2021 * t56082 + 0.34482873909495406156e1_f64 * t38553 + t22158 + 0.13039742234683136782e0_f64 * t9642 * t9601 * t16401 - 0.65198711173415683908e0_f64 * t9678 * t1277 * t16405 + 3.0_f64 / 16.0_f64 * t2011 * t629 * t5 * t55893 - t628 * t629 * t5 * t55933 / 48.0_f64 + 7.0_f64 / 3.0_f64 * t48212 + 7.0_f64 / 36.0_f64 * t48214 + 0.65198711173415683908e0_f64 * t2021 * t56103 - 0.26079484469366273564e0_f64 * t2021 * t56107 + 0.21732903724471894636e-1_f64 * t2021 * t56111 + 0.43465807448943789272e-1_f64 * t636 * t56115 + 0.65198711173415683908e-1_f64 * t636 * t56119;
    (t56110, t56111, t56114, t56115, t56118, t56119, t56122)
}
