//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 958/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk958<F: Float>(t10648: F, t10649: F, t10650: F, t1654: F, t10673: F, t10674: F, t10676: F, t1375: F, t874: F, t10680: F, t10682: F, t122: F, t10675: F, t105: F, t1553: F, t97: F) -> (F, F, F, F, F, F, F) {
    let t37495 = t10648 * t10649 * t10650 * t1654;
    let t37499 = t10673 * t10674 * t1375 * t10676;
    let t37501 = t1654 * t874;
    let t37503 = t10680 * t10682 * t37501;
    let t37505 = t1654 * t122;
    let t37507 = t10673 * t10675 * t37505;
    let t37513 = t97 * t105 * t1553;
    (t37495, t37499, t37501, t37503, t37505, t37507, t37513)
}
