//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1148/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1148<F: Float>(t19309: F, t3227: F, t1092: F, t2861: F, t6557: F, t6498: F, t10245: F, t6496: F, t1021: F, t2825: F, t6497: F, t18443: F, t313: F) -> (F, F, F, F, F, F) {
    let t19310 = t3227 * t19309;
    let t19311 = t1092 * t19310;
    let t19313 = t2861 * t6557;
    let t19315 = t2861 * t6498;
    let t19317 = t10245 * t6496;
    let t19318 = t1021 * t19317;
    let t19319 = t1092 * t19318;
    let t19321 = t2825 * t6497;
    let t19322 = t1092 * t19321;
    let t19324 = t313 * t18443;
    (t19311, t19313, t19315, t19319, t19322, t19324)
}
