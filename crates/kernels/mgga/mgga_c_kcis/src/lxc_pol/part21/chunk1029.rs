//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1029/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1029<F: Float>(t377: F, t5164: F, t13181: F, t389: F, t26938: F, t8072: F, t1096: F, t5096: F, t1021: F, t5086: F, t26929: F, t380: F, t5182: F, t1189: F, t5026: F, t3226: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28014 = t5164 * t377;
    let t28016 = t13181 * t389;
    let t28018 = t26938 * t8072;
    let t28020 = t1096 * t5096;
    let t28022 = t1021 * t5086;
    let t28024 = t380 * t26929;
    let t28025 = t28024 * t5182;
    let t28027 = t5026 * t1189;
    let t28029 = t3226 * t26929;
    (t28014, t28016, t28018, t28020, t28022, t28024, t28025, t28027, t28029)
}
