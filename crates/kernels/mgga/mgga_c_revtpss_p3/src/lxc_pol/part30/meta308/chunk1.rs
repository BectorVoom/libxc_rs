//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1296/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1296<F: Float>(t2661: F, t9981: F, t1384: F, t235: F, t4003: F, t543: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F) -> (F, F, F, F, F, F) {
    let t9982 = t2661 * t9981;
    let t9989 = t1384 * t1384;
    let t9990 = F::new(1.0) / t9989;
    let t9991 = t9990 * t235;
    let t9994 = t4003 * t543;
    let t10001 = t2482 * t4000 * t27;
    let t10003 = t4019 * t221 * t4004;
    (t9982, t9990, t9991, t9994, t10001, t10003)
}
