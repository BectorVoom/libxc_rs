//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2028/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2028<F: Float>(t25981: F, t820: F, t843: F, t4006: F, t2681: F, t7262: F, t1401: F, t25997: F, t9905: F, t533: F, t816: F, t92993: F) -> (F, F, F, F, F) {
    let t94455 = t820 * t25981 * t843;
    let t94456 = t94455 * t4006;
    let t94459 = t820 * t7262 * t2681;
    let t94460 = t94459 * t1401;
    let t94468 = t25997 * t9905;
    let t94471 = t92993 * t533 * t816;
    (t94456, t94459, t94460, t94468, t94471)
}
