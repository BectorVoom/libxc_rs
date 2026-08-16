//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1343/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1343(t10802: f64, t10809: f64, t1359: f64, t1371: f64, t20972: f64, t21000: f64, t2251: f64, t2252: f64, t2267: f64, t2273: f64, t2290: f64, t2291: f64, t2306: f64, t2312: f64, t24896: f64, t24989: f64, t25132: f64, t28967: f64, t29263: f64, t29274: f64, t29288: f64, t29301: f64, t3366: f64, t3386: f64, t4154: f64, t4170: f64, t4181: f64, t4194: f64, t4197: f64, t6641: f64, t6667: f64, t6710: f64, t821: f64, t829: f64, t847: f64, t8709: f64, t8773: f64, t8777: f64, t8857: f64, t8900: f64, t8901: f64, t8916: f64) -> f64 {
    let t29316 = 6.0_f64 * t2273 * t4154 * t2267 + 0.11579025239058625248e4_f64 * t6710 * t4170 * t2252 - 4.0_f64 * t2251 * t1359 * t8900 + 0.34631718211362927517e2_f64 * t8916 * t8773 + 0.20508037716432813315e4_f64 * t24896 * t8777 + 0.35089341735807877242e1_f64 * t2312 * t4181 * t2306 + 0.6233709278045326953e3_f64 * t6667 * t4197 * t2291 - 0.23392894490538584828e1_f64 * t2290 * t1371 * t8709 - 0.10389515463408878255e3_f64 * t6641 * t4197 * t2306 - 0.12304822629859687989e5_f64 * t21000 * t10809 * t2291 - 0.11696447245269292414e1_f64 * t2290 * t4194 * t2306 - 0.10389515463408878255e3_f64 * t6641 * t10802 * t2291 + 0.17315859105681463759e2_f64 * t2312 * t10802 * t2306 + 1.0_f64 * t821 * (t29263 + t29274 + t29288 + t29301) * t829 - t28967 + 0.41016075432865626631e4_f64 * t25132 * t24989 * t847 + 4.0_f64 * t8857 * t3386 + 2.0_f64 * t3366 * t8901 - 2.0_f64 * t20972 * t4154;
    t29316
}
