//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 939/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk939<F: Float>(t136: F, t2476: F, t221: F, t2394: F, t2674: F, t231: F, t243: F, t2645: F, t2662: F, t2661: F, t2652: F, t2656: F, t2482: F, t596: F, t849: F, t2677: F) -> (F, F, F, F, F, F) {
    let t10703 = t2476 * t136;
    let t10705 = t10703 * t221 * t2394;
    let t10706 = t2674 * t10705;
    let t10709 = t243 * t2645 * t231;
    let t10710 = t2662 * t10709;
    let t10711 = t2661 * t10710;
    let t10713 = t2652 * t2656;
    let t10716 = t2482 * t849 * t596;
    let t10717 = t10716 * t2677;
    (t10703, t10706, t10711, t10713, t10716, t10717)
}
