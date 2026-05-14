//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 750/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk750<F: Float>(t34400: F, t34401: F, t41726: F, t12938: F, t2464: F, t587: F, t40514: F, t40517: F, t9065: F, t986: F, t1415: F, t1646: F, t42148: F, t4446: F, t10547: F, t9333: F) -> (F, F, F, F, F, F, F) {
    let t42376 = 0.13803453343411469884e3 * t34400 * t34401 * t41726;
    let t42378 = t587 * t2464 * t12938;
    let t42379 = 0.63904876589867916128e-1 * t42378;
    let t42380 = 0.59584149919750711116e-1 * t40514;
    let t42381 = 0.25561950635947166451e0 * t40517;
    let t42382 = t9065 * t986;
    let t42385 = 0.35750489951850426669e0 * t1415 * t42382 * t1646;
    let t42388 = 0.25025342966295298669e1 * t1415 * t42148 * t4446;
    let t42390 = 0.50050685932590597338e1 * t10547 * t9333;
    (t42376, t42379, t42380, t42381, t42385, t42388, t42390)
}
