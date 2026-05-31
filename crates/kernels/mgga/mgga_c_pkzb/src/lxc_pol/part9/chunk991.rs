//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 991/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk991<F: Float>(t201: F, t7913: F, t199: F, t1162: F, t2476: F, t2340: F, t3147: F, t1217: F, t6117: F, t1185: F, t6205: F, t2192: F, t3070: F) -> (F, F, F, F, F, F, F) {
    let t7914 = t201 * t7913;
    let t7915 = t199 * t7914;
    let t7917 = t1162 * t2476;
    let t7920 = F::cast_from(0.17315859105681463759e2_f64) * t3147 * t2340;
    let t7922 = F::cast_from(0.5848223622634646207e0_f64) * t6117 * t1217;
    let t7924 = F::cast_from(1.0_f64) * t6205 * t1185;
    let t7926 = F::cast_from(2.0_f64) * t2192 * t3070;
    (t7914, t7915, t7917, t7920, t7922, t7924, t7926)
}
