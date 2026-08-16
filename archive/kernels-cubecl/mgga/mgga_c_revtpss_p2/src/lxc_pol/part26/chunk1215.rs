//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1215/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1215<F: Float>(t2103: F, t47567: F, t1364: F, t26338: F, t786: F, t26261: F, t40270: F, t25950: F, t26271: F, t10073: F, t25920: F, t26260: F) -> (F, F, F, F, F) {
    let t96473 = F::cast_from(0.81814717454467823679e-4_f64) * t47567 * t2103;
    let t96486 = t786 * t26338 * t1364;
    let t96491 = F::cast_from(0.96373646535613327356e-3_f64) * t40270 * t26261;
    let t96500 = t25950 * t26271;
    let t96503 = t10073 * t25920 * t26260;
    (t96473, t96486, t96491, t96500, t96503)
}
