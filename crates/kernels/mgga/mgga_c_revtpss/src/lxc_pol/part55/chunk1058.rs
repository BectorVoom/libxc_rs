//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1058/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1058<F: Float>(t120042: F, t1549: F, t31827: F, t31831: F, t31755: F, t31756: F, t4364: F, t4424: F, t125984: F, t25759: F, t126030: F, t1113: F, t7782: F, t1711: F, t7086: F, t125961: F, t27799: F) -> (F, F, F, F, F, F, F, F) {
    let t126396 = t120042 * t1549;
    let t126397 = t31827 * t126396;
    let t126399 = t31831 * t126396;
    let t126403 = t31755 * t4364 * t31756 * t4424;
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    let t127207 = t1113 * t7782;
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    (t126397, t126399, t126403, t127193, t127199, t127207, t127212, t127218)
}
