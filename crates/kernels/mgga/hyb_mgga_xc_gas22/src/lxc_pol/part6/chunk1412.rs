//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1412/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1412<F: Float>(t1112: F, t11217: F, t483: F, t11232: F, t2640: F, t22141: F, t22148: F, t22157: F, t22158: F, t22162: F, t22166: F, t22170: F, t22171: F, t26038: F, t26042: F, t26044: F, t26046: F, t26048: F, t26050: F, t26052: F, t26054: F) -> F {
    let t30518 = t11217 * t483 * t1112;
    let t30520 = t11232 * t2640;
    let t30530 = -t22141 + F::cast_from(80.0_f64) * t26038 - F::cast_from(0.17315859105681463759e2_f64) * t22148 - t22157 - F::cast_from(0.5848223622634646207e0_f64) * t22158 - F::cast_from(0.11696447245269292414e1_f64) * t30518 - F::cast_from(0.17315859105681463759e2_f64) * t30520 - F::cast_from(0.70178683471615754484e1_f64) * t26042 + F::cast_from(0.2077903092681775651e3_f64) * t26044 + F::cast_from(0.46785788981077169656e1_f64) * t26046 - F::cast_from(64.0_f64) * t26048 - F::cast_from(24.0_f64) * t26050 + F::cast_from(120.0_f64) * t26052 + t22162 - F::cast_from(24.0_f64) * t26054 + t22166 + t22170 - F::cast_from(8.0_f64) * t22171;
    t30530
}
