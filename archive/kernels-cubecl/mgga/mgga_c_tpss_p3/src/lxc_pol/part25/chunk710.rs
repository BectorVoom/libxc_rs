//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 710/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk710<F: Float>(t4580: F, t70: F, t1290: F, t1306: F, t2009: F, t4573: F, t4579: F, t48: F, t455: F, t53: F, t2016: F, t60: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t4581 = t4580 * t70;
    let t4584 = t1290 * t1306;
    let t4589 = t2009 * t4573;
    let t4592 = t48 * t4579;
    let t4596 = F::cast_from(1.0_f64) / t53 / t455;
    let t4597 = sigma2 * t4596;
    let t4602 = t2016 * t4573;
    let t4605 = t60 * t4579;
    (t4581, t4584, t4589, t4592, t4597, t4602, t4605)
}
