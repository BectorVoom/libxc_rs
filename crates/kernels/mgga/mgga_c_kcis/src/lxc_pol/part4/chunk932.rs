//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 932/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk932<F: Float>(t12504: F, t11824: F, t569: F, t3733: F, t4291: F, t554: F, t556: F, t11782: F, t577: F, t1527: F, t4121: F, t4248: F, t492: F, t1591: F, t4390: F, t1370: F, t4455: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12505 = t12504 * sigma2;
    let t12520 = t569 * t11824;
    let t12530 = t3733 * t4291;
    let t12534 = 1.0 / t556 / t554;
    let t12542 = t11782 * t577;
    let t12564 = t1527 * t4121;
    let t12568 = t4248 * t492;
    let t12581 = t4390 * t1591;
    let t12605 = t1370 * t4455;
    (t12505, t12520, t12530, t12534, t12542, t12564, t12568, t12581, t12605)
}
