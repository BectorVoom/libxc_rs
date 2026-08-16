//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1178/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1178<F: Float>(t125984: F, t25759: F, t126030: F, t100981: F, t27384: F, t1113: F, t7782: F, t1711: F, t7086: F, t125961: F, t27799: F, t27363: F, t33: F) -> (F, F, F, F, F, F, F) {
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    let t127204 = t100981 * t27384;
    let t127207 = t1113 * t7782;
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    (t127193, t127199, t127204, t127207, t127212, t127218, t127227)
}
