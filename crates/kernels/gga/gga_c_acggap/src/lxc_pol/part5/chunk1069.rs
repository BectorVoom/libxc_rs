//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1069/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1069<F: Float>(t3077: F, t4189: F, t1160: F, t1603: F, t322: F, t407: F, t1410: F, t441: F, t1633: F, t17386: F, t1539: F, t545: F, t943: F) -> (F, F, F, F, F) {
    let t18953 = t3077 * t4189;
    let t18957 = t1160 * t1603 * t322 * t407;
    let t18973 = t441 * t1410;
    let t18977 = t17386 * t1633;
    let t18989 = t1160 * t545 * t943 * t1539;
    (t18953, t18957, t18973, t18977, t18989)
}
