//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1695/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695<F: Float>(t2350: F, t2256: F, t36415: F, t2358: F, t2362: F, t100: F, t101: F, t10217: F, t10227: F, t10229: F, t10232: F, t10233: F, t10236: F, t10237: F, t10241: F, t10246: F, t10250: F, t10344: F, t105: F, t108: F, t2344: F, t2349: F, t2351: F, t2354: F, t2357: F, t46173: F, t46196: F, t656: F, t659: F, t97: F, tau0: F) -> F {
    let t46197 = t2350 * t2350;
    let t46205 = t2256 * t2256;
    let t46212 = F::new(1.0) / t36415;
    let t46213 = t2358 * t2358;
    let t46221 = t2362 * t2362;
    let t46228 = F::new(5.0) / F::new(3.0) * t105 * t108 * t46173 + F::new(6160.0) / F::new(81.0) * tau0 * t10344 * t101 - F::new(8800.0) / F::new(81.0) * t10217 * t659 + F::new(400.0) / F::new(9.0) * t2344 * t2354 - F::new(100.0) / F::new(9.0) * t656 * t10237 - F::new(5.0) / F::new(3.0) * t97 * t100 * t46173 + F::new(800.0) / F::new(27.0) * t2344 * t2351 + F::new(200.0) / F::new(81.0) * t656 * t10229 - F::new(200.0) / F::new(9.0) * t656 * t10233 + F::new(40.0) / F::new(81.0) * t97 * t46196 * t46197 - F::new(20.0) / F::new(9.0) * t97 * t10227 * t2350 * t2256 + F::new(10.0) / F::new(3.0) * t97 * t2349 * t46205 + F::new(40.0) / F::new(9.0) * t97 * t10232 * t10236 + F::new(40.0) / F::new(81.0) * t105 * t46212 * t46213 - F::new(20.0) / F::new(9.0) * t105 * t10241 * t2358 * t2362 + F::new(10.0) / F::new(3.0) * t105 * t2357 * t46221 + F::new(40.0) / F::new(9.0) * t105 * t10246 * t10250;
    t46228
}
