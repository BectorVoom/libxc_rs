//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 699/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk699<F: Float>(t13555: F, t2580: F, t2508: F, t11701: F, t977: F, t3459: F, t8862: F, t3638: F, t7324: F, t5559: F, t2592: F, t3684: F) -> (F, F, F, F, F, F, F, F) {
    let t13556 = t2580 * t13555;
    let t13558 = F::new(0.15381052460284448567e-1) * t2508 * t13556;
    let t13569 = t11701 * t977;
    let t13573 = F::new(4.0) * t8862 * t3459;
    let t13577 = F::new(2.0) * t7324 * t3638;
    let t13578 = t3638 * t977;
    let t13580 = F::new(6.0) * t5559 * t13578;
    let t13584 = t2592 * t3684;
    (t13556, t13558, t13569, t13573, t13577, t13578, t13580, t13584)
}
