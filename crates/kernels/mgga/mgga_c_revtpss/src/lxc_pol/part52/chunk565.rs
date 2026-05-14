//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 565/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk565<F: Float>(t1312: F, t1518: F, t2322: F, t4246: F, t4248: F, t4292: F, t5523: F, t670: F, t1450: F, t1907: F, t198: F, t530: F, t1868: F, t566: F, t532: F, t4147: F) -> (F, F, F, F, F, F) {
    let t5528 = 2.0 * t1312 * t4292 + 2.0 * t1518 * t2322 + 2.0 * t1518 * t5523 + 2.0 * t4248 * t670 + t4246;
    let t5532 = t1907 * t1450;
    let t5536 = t198 * t530;
    let t5537 = t566 * t1868;
    let t5541 = t198 * t532;
    let t5542 = t1907 * t4147;
    (t5528, t5532, t5536, t5537, t5541, t5542)
}
