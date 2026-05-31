//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 815/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk815<F: Float>(t11824: F, t569: F, t3733: F, t4291: F, t554: F, t556: F, t1527: F, t4121: F, t4248: F, t492: F, t12265: F, t577: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t12520 = t569 * t11824;
    let t12530 = t3733 * t4291;
    let t12534 = F::cast_from(1.0_f64) / t556 / t554;
    let t12564 = t1527 * t4121;
    let t12565 = t12564 * sigma2;
    let t12568 = t4248 * t492;
    let t12575 = t12265 * t577;
    (t12520, t12530, t12534, t12564, t12565, t12568, t12575)
}
