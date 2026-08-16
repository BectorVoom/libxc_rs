//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 294/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk294<F: Float>(t1011: F, t1206: F, t476: F, t478: F, t1017: F, t483: F) -> (F, F, F, F, F, F) {
    let t1207 = t1206 * t1011;
    let t1208 = t476 * t476;
    let t1209 = F::cast_from(1.0_f64) / t1208;
    let t1210 = t1209 * t478;
    let t1211 = t483 * t1017;
    let t1212 = t1210 * t1211;
    let t1213 = t1207 * t1212;
    (t1207, t1208, t1209, t1210, t1212, t1213)
}
