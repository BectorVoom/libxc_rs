//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 501/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk501<F: Float>(t12: F, t20: F, t2317: F, t2320: F, t656: F, t22: F, t737: F) -> (F, F, F, F) {
    let t2325 = F::cast_from(1.0_f64)/F::sqrt(t12);
    let t2326 = t2325 * t20;
    let t2327 = t2326 * t2317;
    let t2329 = t656 * t2320;
    let t2331 = t22 * t737;
    (t2326, t2327, t2329, t2331)
}
