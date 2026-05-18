//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1288/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1288<F: Float>(t1208: F, t6323: F, t2295: F, t3113: F, t1201: F, t6121: F, t1196: F, t6288: F, t2279: F, t6282: F, t18580: F, t18706: F, t22355: F, t22359: F, t22361: F, t22380: F, t22385: F, t22390: F, t22393: F, t2273: F, t2298: F, t6275: F, t6276: F, t6290: F, t6324: F, t6338: F, t6341: F, t8120: F, t8153: F, t870: F, t889: F) -> F {
    let t22561 = t6323 * t1208;
    let t22564 = t3113 * t2295;
    let t22567 = t1201 * t6121;
    let t22570 = t6288 * t1196;
    let t22575 = t2279 * t1196;
    let t22578 = t6282 * t1208;
    let t22587 = -F::new(0.31168546390226634766e3) * t22561 * t6338 - F::new(0.35089341735807877242e1) * t22564 * t2298 - F::new(0.10389515463408878255e3) * t22567 * t6324 + F::new(0.6207121550312808036e4) * t22570 * t6290 * t870 * t2273 - t22355 - t22359 - t22361 + F::new(18.0) * t22575 * t6341 - t22380 + F::new(0.30762056574649219974e4) * t22578 * t18580 * t889 + t22385 - t22390 + t22393 + F::new(6.0) * t8120 * t6276 - F::new(0.24828486201251232145e5) * t18706 * t8153 * t6275;
    t22587
}
