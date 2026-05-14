//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 777/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk777<F: Float>(t24: F, t5318: F, t1111: F, t5289: F, t1146: F, t5344: F, t106: F, t1523: F, t5351: F, t8996: F, t5355: F, t9189: F, t3234: F, t438: F, t5311: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15693 = t24 * t5318;
    let t15694 = t1111 * t15693;
    let t15696 = t24 * t5289;
    let t15697 = t1111 * t15696;
    let t15706 = t5344 * t1146;
    let t15713 = t106 * t1523;
    let t15722 = t8996 * t5351;
    let t15736 = t9189 * t5355;
    let t15737 = t3234 * t15736;
    let t15776 = t5311 * t438;
    (t15693, t15694, t15696, t15697, t15706, t15713, t15722, t15736, t15737, t15776)
}
