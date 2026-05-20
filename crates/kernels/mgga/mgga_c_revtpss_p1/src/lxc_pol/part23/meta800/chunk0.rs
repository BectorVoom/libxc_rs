//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2626/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626<F: Float>(t18615: F, t231: F, t243: F, t2661: F, t2662: F, t14923: F, t18478: F, t10811: F, t18334: F, t18629: F, t10777: F, t10779: F, t14671: F, t18637: F) -> (F, F, F, F, F) {
    let t62458 = t2661 * t2662 * t243 * t18615 * t231;
    let t62460 = t14923 * t18478;
    let t62475 = t10811 * t18334;
    let t62494 = t10811 * t18629;
    let t62498 = t10777 * t10779 * t14671 * t18637;
    (t62458, t62460, t62475, t62494, t62498)
}
