//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1148/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1148<F: Float>(t651: F, t7003: F, t2007: F, t670: F, t30: F, t775: F, t1949: F, t212: F) -> (F, F, F, F) {
    let t7005 = F::cast_from(2.0_f64) * t651 * t7003;
    let t7007 = t2007 * t670;
    let t7010 = t30 * t775;
    let t7014 = t212 * t1949;
    (t7005, t7007, t7010, t7014)
}
