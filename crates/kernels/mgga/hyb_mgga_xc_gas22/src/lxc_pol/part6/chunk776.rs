//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 776/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk776<F: Float>(t1359: F, t1371: F, t2251: F, t2273: F, t2290: F, t2312: F, t271: F, t3366: F, t3399: F, t4110: F, t4112: F, t4116: F, t4142: F, t4145: F, t4148: F, t4154: F, t4167: F, t4170: F, t4176: F, t4181: F, t4194: F, t4197: F, t821: F, t840: F) -> F {
    let t4200 = -F::new(0.310907e-1) * t4148 * t271 + F::new(2.0) * t3366 * t1359 - F::new(2.0) * t2251 * t4154 + F::new(1.0) * t821 * t4167 + F::new(0.32163958997385070134e2) * t2273 * t4170 + t4110 - t4112 + t4116 - t4142 - t4145 - F::new(0.19751673498613801407e-1) * t4176 + F::new(0.11696447245269292414e1) * t3399 * t1371 - F::new(0.11696447245269292414e1) * t2290 * t4181 + F::new(0.5848223622634646207e0) * t840 * t4194 + F::new(0.17315859105681463759e2) * t2312 * t4197;
    t4200
}
