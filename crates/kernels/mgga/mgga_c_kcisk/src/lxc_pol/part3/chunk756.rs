//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 756/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk756<F: Float>(t12692: F, t12813: F, t1010: F, t10330: F, t10333: F, t10339: F, t10342: F, t10346: F, t10348: F, t10351: F, t10353: F, t10355: F, t12404: F, t12406: F, t4300: F, t1342: F, sigma0: F) -> (F, F, F, F) {
    let t12814 = t12692 + t12813;
    let t12815 = t1010 * t12814;
    let t12816 = t10330 - t10333 - t10339 + t10342 + t10346 - t10348 + t10351 + t10353 + t10355 - t12404 + t12406 - t12815;
    let t12817 = t4300 * sigma0;
    let t12818 = t12817 * t1342;
    (t12815, t12816, t12817, t12818)
}
