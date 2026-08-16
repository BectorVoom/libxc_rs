//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2610/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610<F: Float>(t1227: F, t13969: F, t15611: F, t15454: F, t4973: F, t49850: F, t11662: F, t11665: F, t15478: F, t15737: F, t44985: F, t44988: F, t44991: F, t44994: F, t44996: F, t4582: F, t48497: F, t4950: F, t51002: F) -> F {
    let t53023 = t1227 * t13969 * t15611;
    let t53026 = t1227 * t13969 * t15454;
    let t53033 = t1227 * t49850 * t4973;
    let t53034 = t53033 / F::cast_from(3456.0_f64);
    let t53037 = -t44985 / F::cast_from(2304.0_f64) - t44988 / F::cast_from(2304.0_f64) - t44991 / F::cast_from(1152.0_f64) - t44994 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1227 * t4582 * t51002 * t48497 - t53023 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t53026 - t44996 * t4950 / F::cast_from(1536.0_f64) - t11665 * t15478 / F::cast_from(768.0_f64) + t53034 + t15737 * t11662 / F::cast_from(512.0_f64);
    t53037
}
