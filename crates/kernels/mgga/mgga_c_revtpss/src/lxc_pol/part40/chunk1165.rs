//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1165/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1165<F: Float>(t3006: F, t4711: F, t11509: F, t1633: F, t2988: F, t4670: F, t953: F, t1622: F, t2962: F, t2944: F, t4673: F, t2970: F, t4669: F, t1634: F, t15127: F, t15168: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15263 = t4711 * t3006;
    let t15266 = t1633 * t11509;
    let t15267 = t15266 * t2988;
    let t15274 = t4670 * t953;
    let t15277 = t1622 * t2962;
    let t15280 = t4673 * t2944;
    let t15283 = t4669 * t2970;
    let t15284 = t15283 * t953;
    let t15287 = t1622 * t2944;
    let t15290 = t1634 * t2988;
    let t15301 = 0.22954444444444444444e0 * t15127;
    let t15312 = 0.27785333333333333334e0 * t15168;
    (t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t15301, t15312)
}
