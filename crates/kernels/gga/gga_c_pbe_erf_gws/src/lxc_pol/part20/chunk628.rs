//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 628/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk628<F: Float>(t143: F, t3644: F, t2864: F, t128: F, t102: F, t120: F, t3637: F, t506: F, t10: F, t1563: F, t127: F, t1511: F, t1519: F, t1540: F, t1555: F, t1561: F, t2879: F, t2891: F, t496: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3645 = t143 * t3644;
    let t3648 = 0.97434166666666666666e0 * t2864;
    let t3649 = t128 * t3644;
    let t3651 = 0.584605e1 * t102 * t3649;
    let t3652 = t120 * t3637;
    let t3654 = 0.2923025e1 * t102 * t3652;
    let t3656 = t506 * t3644;
    let t3657 = t10 * t3656;
    let t3660 = t128 * t3637;
    let t3661 = t10 * t3660;
    let t3665 = t1563 * t3644;
    let t3668 = t506 * t3637;
    let t3671 = -t1511 + t3648 + t1519 + t3651 - t3654 + t1540 + t2879 / 3.0 + 3.0 / 2.0 * t496 * t3657 - t496 * t3661 / 2.0 + t1555 + 0.146904e1 * t2891 + t1561 + 0.587616e1 * t127 * t3665 - 0.146904e1 * t127 * t3668;
    (t3645, t3648, t3649, t3651, t3652, t3654, t3656, t3657, t3660, t3661, t3665, t3668, t3671)
}
