//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 776/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk776(t1359: f64, t1371: f64, t2251: f64, t2273: f64, t2290: f64, t2312: f64, t271: f64, t3366: f64, t3399: f64, t4110: f64, t4112: f64, t4116: f64, t4142: f64, t4145: f64, t4148: f64, t4154: f64, t4167: f64, t4170: f64, t4176: f64, t4181: f64, t4194: f64, t4197: f64, t821: f64, t840: f64) -> f64 {
    let t4200 = -0.310907e-1_f64 * t4148 * t271 + 2.0_f64 * t3366 * t1359 - 2.0_f64 * t2251 * t4154 + 1.0_f64 * t821 * t4167 + 0.32163958997385070134e2_f64 * t2273 * t4170 + t4110 - t4112 + t4116 - t4142 - t4145 - 0.19751673498613801407e-1_f64 * t4176 + 0.11696447245269292414e1_f64 * t3399 * t1371 - 0.11696447245269292414e1_f64 * t2290 * t4181 + 0.5848223622634646207e0_f64 * t840 * t4194 + 0.17315859105681463759e2_f64 * t2312 * t4197;
    t4200
}
