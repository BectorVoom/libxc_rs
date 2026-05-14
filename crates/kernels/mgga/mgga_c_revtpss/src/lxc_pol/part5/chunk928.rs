//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 928/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk928<F: Float>(t2777: F, t4092: F, t2439: F, t1419: F, t3999: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1432: F, t2470: F, t4107: F, t1433: F, t9288: F) -> (F, F, F, F, F, F, F, F) {
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    let t10049 = t3999 * t1419;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    let t10074 = t10073 * t4089;
    let t10098 = t1432 * t4107 * t2470;
    let t10102 = 0.30356481678079769392e-1 * t1432 * t1433 * t9288;
    (t10044, t10049, t10069, t10070, t10073, t10074, t10098, t10102)
}
