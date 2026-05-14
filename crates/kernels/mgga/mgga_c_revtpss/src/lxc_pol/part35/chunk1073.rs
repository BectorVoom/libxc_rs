//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1073/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1073<F: Float>(t102015: F, t114776: F, t114780: F, t114791: F, t115074: F, t115098: F, t115126: F, t115152: F, t115181: F, t115209: F, t115238: F, t115258: F, t115352: F, t115386: F, t1450: F, t1843: F, t2014: F, t2052: F, t2089: F, t22747: F, t25043: F, t25082: F, t28196: F, t28286: F, t29498: F, t29506: F, t30122: F, t30209: F, t30314: F, t30513: F, t30553: F, t30586: F, t30614: F, t30617: F, t34495: F, t4248: F, t508: F, t532: F, t5542: F, t569: F, t5877: F, t6765: F, t7488: F, t7898: F, t7969: F, t8065: F, t8079: F, t98450: F) -> (F,) {
    let t115406 = 3.0 * t2014 * t7488 * t114776 + 6.0 * t7898 * t30617 + 18.0 * t7898 * t30614 + 9.0 * t29506 * t8079 + 18.0 * t25082 * t28286 * t114791 + t2014 * t532 * (t115074 + t115098 + t115126 + t115152 + t115181 + t115209 + t115238 + t115258) * t1450 - 18.0 * t25082 * t34495 * t30122 - 3.0 * t7969 * t6765 - t2052 * t25043 - t115352 * t508 - 3.0 * t30553 * t1843 + t115386 * t569 + 6.0 * t28196 * t28286 * t114780 - 3.0 * t2014 * t30314 * t5542 + 18.0 * t7898 * t30586 - t22747 * t2089 - 3.0 * t5877 * t8065 - 18.0 * t98450 * t30513 + 18.0 * t2014 * t102015 * t29498 - 12.0 * t4248 * t30209;
    (t115406,)
}
