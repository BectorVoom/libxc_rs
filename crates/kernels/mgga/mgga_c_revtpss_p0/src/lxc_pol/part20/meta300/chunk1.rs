//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1184/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1184<F: Float>(t1134: F, t3390: F, t3399: F, t3407: F, t12295: F, t11335: F, t281: F, t414: F, t1139: F, t12322: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F, F, F, F) {
    let t12343 = t3390 * t1134;
    let t12344 = t12343 * t3399;
    let t12346 = t3407 * t1134;
    let t12347 = t12346 * t3399;
    let t12349 = F::cast_from(0.93011851851851851854e0_f64) * t12295;
    let t12351 = t281 * t11335 * t414;
    let t12352 = F::cast_from(0.36514074074074074075e0_f64) * t12351;
    let t12354 = t1139 * t12322;
    let t12356 = F::cast_from(0.19931111111111111111e0_f64) * t12299 + F::cast_from(0.33218518518518518518e0_f64) * t12307 + F::cast_from(0.39862222222222222223e0_f64) * t12297 - F::cast_from(0.59793333333333333333e0_f64) * t12301 - F::cast_from(0.29896666666666666667e0_f64) * t12303 - F::cast_from(0.11958666666666666667e1_f64) * t12310 + F::new(0.17938e1) * t12314 + F::cast_from(0.29896666666666666667e0_f64) * t12320 - F::new(0.28483875e1) * t12344 + F::new(0.46074375e0) * t12347 - t12349 - t12352 + F::new(0.17938e1) * t12317 + F::new(0.3071625e0) * t12354;
    (t12343, t12344, t12346, t12347, t12351, t12354, t12356)
}
