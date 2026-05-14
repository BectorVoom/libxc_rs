//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1370/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1370<F: Float>(t27307: F, t27329: F, t27355: F, t27392: F, t881: F, t890: F, t898: F, t8009: F, t8192: F, t22823: F, t8195: F, t26878: F, t26883: F, t26885: F, t26888: F, t26890: F, t26892: F, t26895: F, t26898: F, t26900: F, t27253: F) -> (F, F, F, F, F) {
    let t27394 = t27307 + t27329 + t27355 + t27392;
    let t27398 = 0.5848223622634646207e0 * t898 * t881 * t27394 * t890;
    let t27400 = 4.0 * t8009 * t8192;
    let t27402 = 0.19298375398431042081e3 * t22823 * t8195;
    let t27403 = -t26878 + t26883 + t26885 + t26888 - t26890 - t26892 - t26895 + t26898 - t26900 + t27253 - t27398 - t27400 - t27402;
    (t27394, t27398, t27400, t27402, t27403)
}
