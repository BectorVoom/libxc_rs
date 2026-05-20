//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2542/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542<F: Float>(t52035: F, t52037: F, t11223: F, t1678: F, t1041: F, t11262: F, t4868: F, t3201: F, t4794: F, t4798: F, t343: F, t44: F, t816: F) -> (F, F, F, F, F, F, F) {
    let t53252 = F::cast_from(0.39511111111111111112e-1_f64) * t52035;
    let t53253 = F::cast_from(0.13170370370370370371e-1_f64) * t52037;
    let t53281 = t11223 * t1678;
    let t53293 = t1041 * t11262 * t4868;
    let t53294 = F::cast_from(0.14291339372689912324e-3_f64) * t53293;
    let t53300 = t4794 * t3201;
    let t53317 = t4798 * t3201;
    let t53318 = F::cast_from(0.14291339372689912324e-3_f64) * t53317;
    let t53320 = t44 * t343 * t816;
    (t53252, t53253, t53281, t53294, t53300, t53318, t53320)
}
