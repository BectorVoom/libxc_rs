//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1438/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1438<F: Float>(t17306: F, t3754: F, t10308: F, t1466: F, t2246: F, t5812: F, t11064: F, t6075: F, t37: F, t5940: F, t2609: F, t5825: F, t706: F) -> (F, F, F, F, F, F) {
    let t60019 = t17306 * t3754;
    let t60224 = t1466 * t10308;
    let t60673 = t5812 * t2246;
    let t61033 = t6075 * t11064;
    let t61037 = t37 * t5940;
    let t61090 = t706 * t2609 * t5825;
    (t60019, t60224, t60673, t61033, t61037, t61090)
}
