//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1171/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1171<F: Float>(t4496: F, t4497: F, t959: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F) -> (F, F, F, F, F) {
    let t4498 = t4496 * t4497;
    let t4500 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t4498;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    let t4509 = t60 * t2978;
    (t4498, t4500, t4506, t4507, t4509)
}
