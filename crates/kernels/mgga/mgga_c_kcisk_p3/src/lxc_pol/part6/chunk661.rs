//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 661/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk661<F: Float>(t772: F, t79: F, t9206: F, t781: F, t2063: F, t2642: F, t5491: F, t1775: F, t5497: F, t7715: F, t9155: F, t2021: F, t1586: F) -> (F, F, F, F, F, F, F, F, F) {
    let t783 = F::cast_from(0.0_f64) < t772;
    let t9207 = t79 * t9206;
    let t9208 = t9207 * t781;
    let t9212 = t2063 * t2642;
    let t9213 = t5491 * t9212;
    let t9214 = t1775 * t9213;
    let t9217 = t5497 * t7715;
    let t9218 = t1775 * t9217;
    let t9226 = piecewise3::<F>(t783, t9155, -t9155);
    let t9227 = t2021 * t9226;
    let t9228 = t1586 * t9227;
    (t9207, t9208, t9213, t9214, t9217, t9218, t9226, t9227, t9228)
}
