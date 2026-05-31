//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1260/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1260<F: Float>(t26727: F, t2927: F, t1128: F, t9761: F, t2881: F, t2868: F, t10154: F, t10516: F, t11261: F, t11263: F, t11265: F, t8586: F, t8951: F, t8953: F, t9315: F, t9316: F, t9412: F, t9413: F, t9415: F) -> (F, F, F, F, F) {
    let t26886 = t2927 * t26727;
    let t26927 = t9761 * t1128;
    let t26973 = t2881 * t26727;
    let t26976 = t2868 * t26727;
    let t27002 = F::cast_from(2.0_f64) * t10154 + F::cast_from(2.0_f64) * t10516 + F::cast_from(2.0_f64) * t9315 + F::cast_from(2.0_f64) * t9316 + F::cast_from(2.0_f64) * t9412 + F::cast_from(4.0_f64) * t8951 + F::cast_from(2.0_f64) * t8953 + F::cast_from(4.0_f64) * t9413 + F::cast_from(2.0_f64) * t9415 + F::cast_from(4.0_f64) * t11263 + F::cast_from(2.0_f64) * t11265 + F::cast_from(2.0_f64) * t8586 + F::cast_from(4.0_f64) * t11261;
    (t26886, t26927, t26973, t26976, t27002)
}
