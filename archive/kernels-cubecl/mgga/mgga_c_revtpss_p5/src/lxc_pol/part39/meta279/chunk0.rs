//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1020/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1020<F: Float>(t4021: F, t9976: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1384: F, t235: F, t4003: F, t543: F, t2482: F, t27: F, t4000: F) -> (F, F, F, F, F, F) {
    let t9977 = t9976 * t4021;
    let t9979 = t1412 * t1398;
    let t9980 = t9979 * t3938;
    let t9981 = t3992 * t9980;
    let t9982 = t2661 * t9981;
    let t9989 = t1384 * t1384;
    let t9990 = F::cast_from(1.0_f64) / t9989;
    let t9991 = t9990 * t235;
    let t9994 = t4003 * t543;
    let t10001 = t2482 * t4000 * t27;
    (t9977, t9982, t9990, t9991, t9994, t10001)
}
