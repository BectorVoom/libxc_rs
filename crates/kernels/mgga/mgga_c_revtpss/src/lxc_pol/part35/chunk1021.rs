//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1021/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1021<F: Float>(t1904: F, t2439: F, t26358: F, t10073: F, t25937: F, t7282: F, t8085: F, t102385: F, t94383: F, t26260: F, t27836: F, t1385: F, t198: F, t206: F, t8019: F, t136: F, t2457: F, t8006: F) -> (F, F, F, F, F, F, F) {
    let t102582 = t2439 * t26358 * t1904;
    let t102610 = t10073 * t7282 * t25937 * t8085;
    let t102629 = t94383 * t102385;
    let t102636 = t10073 * t27836 * t26260;
    let t102656 = t1385 * t8085;
    let t102888 = t198 * t206 * t8019;
    let t102980 = t8006 * t136 * t2457;
    (t102582, t102610, t102629, t102636, t102656, t102888, t102980)
}
