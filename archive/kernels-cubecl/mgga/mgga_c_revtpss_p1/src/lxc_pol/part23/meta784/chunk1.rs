//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2594/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594<F: Float>(t2246: F, t5812: F, t1469: F, t627: F, t72: F, t10605: F, t18539: F, t11064: F, t6075: F, t37: F, t5940: F, t2609: F, t5825: F, t706: F) -> (F, F, F, F, F, F) {
    let t60673 = t5812 * t2246;
    let t60823 = t1469 * t627 * t72;
    let t61020 = t10605 * t18539;
    let t61033 = t6075 * t11064;
    let t61037 = t37 * t5940;
    let t61090 = t706 * t2609 * t5825;
    (t60673, t60823, t61020, t61033, t61037, t61090)
}
