//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 832/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk832<F: Float>(t313: F, t6352: F, t934: F, t3293: F, t1045: F, t6353: F, t4642: F, t1098: F, t6598: F, t6602: F, t1646: F, t1727: F) -> (F, F, F, F, F, F, F) {
    let t18595 = t313 * t6352;
    let t18596 = t18595 * t934;
    let t18597 = t3293 * t18596;
    let t18600 = t6353 * t1045;
    let t18601 = t4642 * t18600;
    let t18606 = t1098 * t6598;
    let t18608 = t1098 * t6602;
    let t18613 = t1646 * t1727;
    (t18596, t18597, t18600, t18601, t18606, t18608, t18613)
}
