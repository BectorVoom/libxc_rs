//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1412/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1412(t1112: f64, t11217: f64, t483: f64, t11232: f64, t2640: f64, t22141: f64, t22148: f64, t22157: f64, t22158: f64, t22162: f64, t22166: f64, t22170: f64, t22171: f64, t26038: f64, t26042: f64, t26044: f64, t26046: f64, t26048: f64, t26050: f64, t26052: f64, t26054: f64) -> f64 {
    let t30518 = t11217 * t483 * t1112;
    let t30520 = t11232 * t2640;
    let t30530 = -t22141 + 80.0_f64 * t26038 - 0.17315859105681463759e2_f64 * t22148 - t22157 - 0.5848223622634646207e0_f64 * t22158 - 0.11696447245269292414e1_f64 * t30518 - 0.17315859105681463759e2_f64 * t30520 - 0.70178683471615754484e1_f64 * t26042 + 0.2077903092681775651e3_f64 * t26044 + 0.46785788981077169656e1_f64 * t26046 - 64.0_f64 * t26048 - 24.0_f64 * t26050 + 120.0_f64 * t26052 + t22162 - 24.0_f64 * t26054 + t22166 + t22170 - 8.0_f64 * t22171;
    t30530
}
