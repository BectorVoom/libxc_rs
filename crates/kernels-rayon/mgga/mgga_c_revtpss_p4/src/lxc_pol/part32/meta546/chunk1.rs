//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1861/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861(t25899: f64, t96245: f64, t1358: f64, t2439: f64, t7506: f64, t785: f64, t26276: f64, t9285: f64, t25944: f64, t136: f64, t2457: f64, t7531: f64) -> (f64, f64, f64, f64, f64) {
    let t96246 = t25899 * t96245;
    let t96253 = t2439 * t785 * t7506 * t1358;
    let t96255 = t26276 * t9285;
    let t96257 = 0.68540937416128198417e-2_f64 * t25944 * t96255;
    let t96259 = t7531 * t136 * t2457;
    (t96246, t96253, t96255, t96257, t96259)
}
