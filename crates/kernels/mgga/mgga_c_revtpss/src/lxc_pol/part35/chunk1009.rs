//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1009/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1009<F: Float>(t26276: F, t9285: F, t25944: F, t7514: F, t9288: F, t7289: F, t94471: F, t94473: F, t94476: F, t94483: F, t94522: F, t94525: F, t94568: F, t94570: F, t7284: F, t26069: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t96255 = t26276 * t9285;
    let t96257 = 0.68540937416128198417e-2 * t25944 * t96255;
    let t96282 = t7514 * t9288;
    let t96284 = 0.39982213492741449076e-1 * t7289 * t96282;
    let t96321 = 455.0 / 648.0 * t94471;
    let t96322 = 0.51384669507166276316e-2 * t94473;
    let t96323 = 0.3252886739816735289e-3 * t94476;
    let t96326 = 0.18295201011342718161e-3 * t94483;
    let t96341 = 0.15117061203111996147e0 * t94522;
    let t96342 = 0.80328230880474379779e-6 * t94525;
    let t96358 = 0.45178982497454656792e-6 * t94568;
    let t96359 = 0.28900264064772933812e-2 * t94570;
    let t96374 = 0.22487184191643109717e-1 * t7284 * t96282;
    let t96401 = 0.91399340044406952588e-2 * t26069 * t96255;
    (t96257, t96284, t96321, t96322, t96323, t96326, t96341, t96342, t96358, t96359, t96374, t96401)
}
