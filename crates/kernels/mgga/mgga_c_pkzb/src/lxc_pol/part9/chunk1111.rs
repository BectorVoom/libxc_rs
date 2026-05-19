//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1111/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1111<F: Float>(t2288: F, t2295: F, t2317: F, t2249: F, t2278: F, t18439: F, t18442: F, t6141: F, t828: F, t2189: F, t2196: F, t6352: F, t862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18711 = t2288 * t2295;
    let t18740 = t2288 * t2317;
    let t18747 = t2249 * t2278;
    let t18750 = F::cast_from(0.17757530864197530864e0_f64) * t18439;
    let t18765 = F::cast_from(0.5356037037037037037e1_f64) * t18439;
    let t18766 = F::cast_from(0.16979925925925925926e1_f64) * t18442;
    let t18790 = t828 * t6141;
    let t18796 = t2189 * t2196;
    let t18799 = t6352 * t862;
    (t18711, t18740, t18747, t18750, t18765, t18766, t18790, t18796, t18799)
}
