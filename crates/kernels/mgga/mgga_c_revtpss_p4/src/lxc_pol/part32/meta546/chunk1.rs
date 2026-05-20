//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1861/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1861<F: Float>(t25899: F, t96245: F, t1358: F, t2439: F, t7506: F, t785: F, t26276: F, t9285: F, t25944: F, t136: F, t2457: F, t7531: F) -> (F, F, F, F, F) {
    let t96246 = t25899 * t96245;
    let t96253 = t2439 * t785 * t7506 * t1358;
    let t96255 = t26276 * t9285;
    let t96257 = F::cast_from(0.68540937416128198417e-2_f64) * t25944 * t96255;
    let t96259 = t7531 * t136 * t2457;
    (t96246, t96253, t96255, t96257, t96259)
}
