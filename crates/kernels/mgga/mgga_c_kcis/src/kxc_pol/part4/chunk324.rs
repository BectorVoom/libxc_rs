//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 324/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk324<F: Float>(t1142: F, t1203: F, t920: F, t924: F, t401: F) -> (F, F, F, F, F) {
    let t1204 = t1142 * t1203;
    let t1205 = F::cast_from(0.17123333333333333333e-1_f64) * t920;
    let t1207 = -t1205 - F::cast_from(0.17123333333333333333e-1_f64) * t924;
    let t1210 = t401 * t401;
    let t1211 = F::new(1.0) / t1210;
    (t1204, t1205, t1207, t1210, t1211)
}
