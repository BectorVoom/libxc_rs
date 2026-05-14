//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1103/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1103<F: Float>(t2034: F, t56073: F, t13214: F, t4599: F, t6931: F, t13174: F, t4595: F, t1256: F, t48577: F, t1271: F, t48590: F, t162: F, t48308: F, t13248: F, t1277: F, t16401: F, t16405: F, t2011: F, t2021: F, t22158: F, t38553: F, t48212: F, t48214: F, t5: F, t55893: F, t55933: F, t628: F, t629: F, t636: F, t9601: F, t9642: F, t9678: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t56074 = t2034 * t56073;
    let t56077 = t13214 * t4599;
    let t56078 = t6931 * t56077;
    let t56081 = t13174 * t4595;
    let t56082 = t2034 * t56081;
    let t56102 = t13174 * t4599;
    let t56103 = t6931 * t56102;
    let t56106 = t48577 * t1256;
    let t56107 = t2034 * t56106;
    let t56110 = t48590 * t1271;
    let t56111 = t162 * t56110;
    let t56114 = t48308 * t1256;
    let t56115 = t2034 * t56114;
    let t56118 = t13248 * t4595;
    let t56119 = t2034 * t56118;
    let t56122 = 0.43465807448943789272e-1 * t636 * t56074 - 0.32599355586707841954e0 * t636 * t56078 - 0.13039742234683136782e0 * t2021 * t56082 + 0.34482873909495406156e1 * t38553 + t22158 + 0.13039742234683136782e0 * t9642 * t9601 * t16401 - 0.65198711173415683908e0 * t9678 * t1277 * t16405 + 3.0 / 16.0 * t2011 * t629 * t5 * t55893 - t628 * t629 * t5 * t55933 / 48.0 + 7.0 / 3.0 * t48212 + 7.0 / 36.0 * t48214 + 0.65198711173415683908e0 * t2021 * t56103 - 0.26079484469366273564e0 * t2021 * t56107 + 0.21732903724471894636e-1 * t2021 * t56111 + 0.43465807448943789272e-1 * t636 * t56115 + 0.65198711173415683908e-1 * t636 * t56119;
    (t56074, t56077, t56078, t56081, t56082, t56102, t56103, t56106, t56107, t56110, t56111, t56114, t56115, t56118, t56119, t56122)
}
