//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1128/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1128(t94696: f64, t96204: f64, t10115: f64, t2099: f64, t7493: f64, t9292: f64, t2097: f64, t9646: f64, t9648: f64, t25875: f64, t96186: f64, t26276: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96206 = 0.43639970290213137151e-3_f64 * t94696 * t96204;
    let t96210 = 0.11044544084478153697e-3_f64 * t10115 * t2099;
    let t96218 = 0.17073386770573548589e-1_f64 * t9292 * t7493;
    let t96230 = 0.19637199382202157274e-3_f64 * t9646 * t2097 * t9648;
    let t96236 = t25875 * t96186;
    let t96255 = t26276 * t9285;
    (t96206, t96210, t96218, t96230, t96236, t96255)
}
