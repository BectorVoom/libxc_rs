//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2731/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2731<F: Float>(t15045: F, t2435: F, t15048: F, t2471: F, t15008: F, t10996: F, t14990: F, t41070: F, t14939: F, t212: F, t689: F, t780: F) -> (F, F, F, F, F) {
    let t50218 = t2435 * t15045;
    let t50219 = F::cast_from(0.21951497276451705329e-1_f64) * t50218;
    let t50220 = t15048 * t2471;
    let t50221 = F::cast_from(0.39029762157531132076e-1_f64) * t50220;
    let t50222 = t2435 * t15008;
    let t50223 = F::cast_from(0.21951497276451705329e-1_f64) * t50222;
    let t50227 = t41070 * t14990 * t10996;
    let t50232 = t689 * t212 * t14939 * t780;
    (t50219, t50221, t50223, t50227, t50232)
}
