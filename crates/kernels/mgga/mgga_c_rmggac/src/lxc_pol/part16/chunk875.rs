//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 875/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk875<F: Float>(t40918: F, t40970: F, t40976: F, t41041: F, t41057: F, t41114: F, t41128: F, t41438: F, t2227: F, t551: F, t1614: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44075 = F::cast_from(0.10909864661698136692e0_f64) * t40918;
    let t44093 = F::cast_from(0.10909864661698136692e0_f64) * t40970;
    let t44095 = F::cast_from(0.1454648621559751559e0_f64) * t40976;
    let t44110 = F::cast_from(0.36366215538993788974e-1_f64) * t41041;
    let t44114 = F::cast_from(0.10909864661698136692e0_f64) * t41057;
    let t44143 = F::cast_from(0.15965655602485078085e0_f64) * t41114;
    let t44145 = F::cast_from(0.3193131120497015617e0_f64) * t41128;
    let t44169 = F::cast_from(0.3193131120497015617e0_f64) * t41438;
    let t44187 = t2227 * t551;
    let t44194 = t698 * t1614;
    (t44075, t44093, t44095, t44110, t44114, t44143, t44145, t44169, t44187, t44194)
}
