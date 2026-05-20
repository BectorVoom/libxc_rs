//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1199/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1199<F: Float>(t2097: F, t22: F, t25937: F, t94696: F, t10115: F, t2099: F, t26072: F, t26292: F, t7493: F, t9292: F, t136: F, t137: F) -> (F, F, F, F, F, F) {
    let t96204 = t25937 * t2097 * t22;
    let t96206 = F::cast_from(0.43639970290213137151e-3_f64) * t94696 * t96204;
    let t96210 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2099;
    let t96211 = t26072 * t26292;
    let t96218 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7493;
    let t96220 = t2097 * t136 * t137;
    (t96204, t96206, t96210, t96211, t96218, t96220)
}
