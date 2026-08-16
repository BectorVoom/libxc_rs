//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2237/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2237<F: Float>(t193: F, t776: F, t12908: F, t13127: F, t3966: F, t4194: F, t607: F, t750: F, t12606: F, t184: F, t4202: F, t9912: F) -> (F, F, F, F, F) {
    let t46341 = t193 * t776;
    let t46345 = F::cast_from(72.0_f64) * t12908 * t13127;
    let t46348 = t4194 * t750 * t3966 * t607;
    let t46349 = F::cast_from(72.0_f64) * t46348;
    let t46353 = F::cast_from(36.0_f64) * t4194 * t184 * t12606 * t607;
    let t46355 = F::cast_from(12.0_f64) * t9912 * t4202;
    (t46341, t46345, t46349, t46353, t46355)
}
