//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 298/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk298<F: Float>(t1634: F, t1657: F, t599: F) -> (F, F, F) {
    let t1678 = F::cast_from(0.301925e0_f64) * t1634;
    let t1681 = F::cast_from(0.82785e-1_f64) * t1657;
    let t1685 = F::cast_from(1.0_f64) / t599;
    (t1678, t1681, t1685)
}
