//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 856/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk856<F: Float>(t1219: F, t8172: F, t5357: F, t7607: F, t5378: F, t7624: F, t1785: F, t7623: F, t3670: F, t2133: F, t816: F, t1224: F, t65: F, t5052: F, t1266: F, t1808: F, t26821: F, t26822: F, t26832: F, t26836: F, t26852: F, t26867: F, t5386: F, t5407: F) -> (F, F, F) {
    let t29027 = t8172 * t1219;
    let t29031 = t7607 * t5357;
    let t29034 = t7624 * t5378;
    let t29037 = t1785 * t7623;
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29049 = t29048 * t5052;
    let t29052 = -t26821 + 0.28582678745379824648e-3 * t26822 - t29031 / 864.0 - 0.28582678745379824648e-3 * t26832 - 0.19055119163586549765e-3 * t29034 - t26836 / 864.0 - 0.28582678745379824648e-3 * t29037 * t1266 + 0.85748036236139473944e-3 * t29040 * t5386 - 0.28582678745379824648e-3 * t26852 * t1808 - 0.28582678745379824648e-3 * t26867 * t5407 - t29047 * t29049 / 144.0;
    (t29027, t29047, t29052)
}
