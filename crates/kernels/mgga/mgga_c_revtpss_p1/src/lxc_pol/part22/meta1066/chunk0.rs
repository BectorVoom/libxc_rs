//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3816/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3816<F: Float>(t1317: F, t22195: F, t48235: F, t48237: F, t48240: F, t48243: F, t46975: F, t46977: F, t46983: F, t1320: F, t22193: F, t10186: F, t198: F, t39531: F, t49544: F, t6836: F, t6930: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t73360 = t1317 * t22195;
    let t73361 = F::new(8.0) * t73360;
    let t73364 = F::new(80.0) * t48235;
    let t73365 = F::new(8.0) * t48237;
    let t73366 = F::new(4.0) * t48240;
    let t73367 = F::new(2.0) * t48243;
    let t73371 = F::new(160.0) * t46975;
    let t73372 = F::new(240.0) * t46977;
    let t73373 = F::new(8.0) * t46983;
    let t73374 = t1320 * t22193;
    let t73375 = F::new(8.0) * t73374;
    let t73376 = F::new(6.0) * t10186 * t198 * t6836 + F::new(12.0) * t49544 * t6930 + t39531 + t73361 + t73364 - t73365 + t73366 + t73367 - t73371 - t73372 - t73373 - t73375;
    (t73361, t73364, t73365, t73366, t73367, t73371, t73372, t73373, t73375, t73376)
}
