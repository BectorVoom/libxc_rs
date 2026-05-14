//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 938/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk938<F: Float>(t169: F, t18374: F, t18375: F, t234: F, t1767: F, t3217: F, t1262: F, t1851: F, t2153: F, t2539: F, t9275: F, t1295: F, t914: F, t2169: F, t2210: F, t2794: F, t2146: F, t2537: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t18376 = -t18374 + t18375;
    let t18401 = piecewise3(t170, 0.0, -t18376);
    let t18402 = t234 * t18401;
    let t19575 = t3217 * t1767;
    let t20572 = t1851 * t1262;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    let t26392 = 6.0 * t26391;
    let t26393 = t914 * t1295;
    let t26394 = t2169 * t26393;
    let t26395 = t26394 / 8.0;
    let t26396 = t2794 * t2210;
    let t26397 = t26396 / 8.0;
    let t26398 = t2146 * t2537;
    (t18401, t18402, t19575, t20572, t26390, t26391, t26392, t26395, t26397, t26398)
}
