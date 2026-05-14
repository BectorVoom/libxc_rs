//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1016/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1016<F: Float>(t125405: F, t125407: F, t125409: F, t125415: F, t125417: F, t125431: F, t125432: F, t129281: F, t129283: F, t129285: F, t1502: F, t1843: F, t2127: F, t2163: F, t29337: F, t29422: F, t33375: F, t33550: F, t4246: F, t5517: F, t7584: F, t7683: F, t8152: F, t8233: F, t8917: F, t8964: F) -> (F,) {
    let t131216 = -t1502 * t33550 - t1843 * t33375 - 2.0 * t2127 * t29337 - 2.0 * t2163 * t29422 - t4246 * t8964 - t5517 * t8917 - 2.0 * t7584 * t8233 - 2.0 * t7683 * t8152 - t125405 - t125407 - t125409 + t125415 - t125417 - t125431 - t125432 + 2.0 * t129281 + 2.0 * t129283 - 6.0 * t129285;
    (t131216,)
}
