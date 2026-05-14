//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 853/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk853<F: Float>(t17419: F, t1621: F, t1724: F, t1793: F, t5109: F, t639: F, t1620: F, t5111: F, t617: F, t1635: F, t5470: F, t1645: F, t1627: F, t5485: F, t1630: F, t5484: F) -> (F, F, F, F, F, F, F) {
    let t17420 = 32.0 / 15.0 * t17419;
    let t17425 = 24.0 / 5.0 * t639 * t1621 * t5109 * t1793 * t1724;
    let t17430 = 32.0 / 5.0 * t1620 * t1621 * t5109 * t5111 * t617;
    let t17432 = 8.0 / 15.0 * t5470 * t1635;
    let t17434 = 8.0 / 9.0 * t5470 * t1645;
    let t17436 = 16.0 / 45.0 * t1627 * t5485;
    let t17438 = t639 * t1630 * t5484;
    (t17420, t17425, t17430, t17432, t17434, t17436, t17438)
}
