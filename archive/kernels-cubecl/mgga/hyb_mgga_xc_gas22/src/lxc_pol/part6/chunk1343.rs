//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1343/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1343<F: Float>(t10802: F, t10809: F, t1359: F, t1371: F, t20972: F, t21000: F, t2251: F, t2252: F, t2267: F, t2273: F, t2290: F, t2291: F, t2306: F, t2312: F, t24896: F, t24989: F, t25132: F, t28967: F, t29263: F, t29274: F, t29288: F, t29301: F, t3366: F, t3386: F, t4154: F, t4170: F, t4181: F, t4194: F, t4197: F, t6641: F, t6667: F, t6710: F, t821: F, t829: F, t847: F, t8709: F, t8773: F, t8777: F, t8857: F, t8900: F, t8901: F, t8916: F) -> F {
    let t29316 = F::cast_from(6.0_f64) * t2273 * t4154 * t2267 + F::cast_from(0.11579025239058625248e4_f64) * t6710 * t4170 * t2252 - F::cast_from(4.0_f64) * t2251 * t1359 * t8900 + F::cast_from(0.34631718211362927517e2_f64) * t8916 * t8773 + F::cast_from(0.20508037716432813315e4_f64) * t24896 * t8777 + F::cast_from(0.35089341735807877242e1_f64) * t2312 * t4181 * t2306 + F::cast_from(0.6233709278045326953e3_f64) * t6667 * t4197 * t2291 - F::cast_from(0.23392894490538584828e1_f64) * t2290 * t1371 * t8709 - F::cast_from(0.10389515463408878255e3_f64) * t6641 * t4197 * t2306 - F::cast_from(0.12304822629859687989e5_f64) * t21000 * t10809 * t2291 - F::cast_from(0.11696447245269292414e1_f64) * t2290 * t4194 * t2306 - F::cast_from(0.10389515463408878255e3_f64) * t6641 * t10802 * t2291 + F::cast_from(0.17315859105681463759e2_f64) * t2312 * t10802 * t2306 + F::cast_from(1.0_f64) * t821 * (t29263 + t29274 + t29288 + t29301) * t829 - t28967 + F::cast_from(0.41016075432865626631e4_f64) * t25132 * t24989 * t847 + F::cast_from(4.0_f64) * t8857 * t3386 + F::cast_from(2.0_f64) * t3366 * t8901 - F::cast_from(2.0_f64) * t20972 * t4154;
    t29316
}
