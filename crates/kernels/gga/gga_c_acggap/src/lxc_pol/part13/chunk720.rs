//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 720/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk720<F: Float>(t7932: F, t8406: F, t7942: F, t2341: F, t463: F, t8004: F, t2147: F, t2331: F, t322: F, t2132: F, t2138: F, t309: F, t2131: F, t157: F, t406: F, t2152: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8407 = t7932 * t8406;
    let t8408 = t7942 * t8407;
    let t8410 = t2341 * t463;
    let t8411 = t8004 * t8410;
    let t8415 = t2147 * t2331 * t463;
    let t8418 = t2331 * t322;
    let t8419 = t2132 * t8418;
    let t8420 = t2138 * t8419;
    let t8422 = t2331 * t309;
    let t8423 = t2132 * t8422;
    let t8424 = t2131 * t8423;
    let t8427 = t2331 * t406 * t157;
    let t8428 = t2152 * t8427;
    (t8407, t8408, t8411, t8415, t8418, t8419, t8420, t8422, t8423, t8424, t8428)
}
