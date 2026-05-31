//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 852/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk852<F: Float>(t6158: F, t834: F, t6087: F, t336: F, t6150: F, t2215: F, t836: F, t2209: F, t841: F, t218: F, t344: F, t5555: F) -> (F, F, F, F, F, F, F) {
    let t6159 = t834 * t6158;
    let t6161 = F::cast_from(0.93011851851851851854e0_f64) * t6087;
    let t6165 = F::cast_from(1.0_f64)/pow_3_2::<F>(t336);
    let t6166 = t6165 * t6150;
    let t6168 = t2215 * t836;
    let t6169 = t6168 * t2209;
    let t6171 = t841 * t6158;
    let t6174 = t218 * t5555 * t344;
    (t6159, t6161, t6165, t6166, t6169, t6171, t6174)
}
