//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 861/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk861<F: Float>(t11315: F, t923: F, t11156: F, t2908: F, t141: F, t11165: F, t930: F, t2912: F, t698: F, t11151: F, t11160: F, t11132: F, t240: F, t624: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11316 = t923 * t11315;
    let t11318 = t2908 * t11156;
    let t11319 = t141 * t11318;
    let t11321 = t930 * t11165;
    let t11322 = t141 * t11321;
    let t11326 = t698 * t2912;
    let t11328 = t2908 * t11151;
    let t11329 = t141 * t11328;
    let t11331 = t930 * t11160;
    let t11332 = t141 * t11331;
    let t11334 = 0.93011851851851851854e0 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    (t11316, t11319, t11322, t11326, t11329, t11332, t11334, t11335, t11337)
}
