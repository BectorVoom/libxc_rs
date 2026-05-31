//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 996/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk996<F: Float>(t1242: F, t3497: F, t1236: F, t3643: F, t1239: F, t3609: F, t1238: F, t413: F, t10471: F, t1281: F, t3662: F, t1278: F, t3668: F) -> (F, F, F, F, F, F, F) {
    let t11100 = t1242 * t3497;
    let t11151 = t1236 * t3643;
    let t11178 = t3609 * t1239;
    let t11181 = t1238 * t1238;
    let t11182 = F::cast_from(1.0_f64) / t11181;
    let t11183 = t413 * t11182;
    let t11209 = F::cast_from(0.51588271604938271604e-3_f64) * t10471;
    let t11220 = t3662 * t1281;
    let t11223 = t1278 * t3668;
    (t11100, t11151, t11178, t11183, t11209, t11220, t11223)
}
