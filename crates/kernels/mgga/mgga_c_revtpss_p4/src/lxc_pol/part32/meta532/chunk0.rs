//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1837/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1837<F: Float>(t3981: F, t94443: F, t25981: F, t820: F, t843: F, t2681: F, t7262: F, t1401: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F) -> (F, F, F, F, F, F) {
    let t94444 = t94443 * t3981;
    let t94455 = t820 * t25981 * t843;
    let t94459 = t820 * t7262 * t2681;
    let t94460 = t94459 * t1401;
    let t94471 = t92993 * t533 * t816;
    let t94473 = t7259 * t9709;
    (t94444, t94455, t94459, t94460, t94471, t94473)
}
