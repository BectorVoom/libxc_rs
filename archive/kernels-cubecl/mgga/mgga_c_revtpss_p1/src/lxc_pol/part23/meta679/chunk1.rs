//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2419/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2419<F: Float>(t12627: F, t1284: F, t3624: F, t12640: F, t3555: F, t3781: F, t5330: F, t3617: F, t675: F, t1263: F, t215: F, t1122: F, t1261: F, t247: F) -> (F, F, F, F, F, F) {
    let t44609 = t12627 * t1284 * t3624;
    let t44624 = t12640 * t1284 * t3624;
    let t44664 = t3555 * t3781 * t5330;
    let t44693 = t675 * t3617;
    let t44701 = t215 * t1263;
    let t44704 = t1261 * t247 * t44701 * t1122;
    (t44609, t44624, t44664, t44693, t44701, t44704)
}
