//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1392/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1392<F: Float>(t1036: F, t23551: F, t23562: F, t343: F, t83032: F, t210: F, t23322: F, t23460: F, t995: F, t3: F, t9258: F, t23452: F, t6739: F, t6741: F) -> (F, F, F, F, F, F) {
    let t83082 = t23551 * t1036;
    let t83085 = t23562 * t83032 * t343;
    let t83092 = t23322 * t210;
    let t83098 = t23460 * t995;
    let t83100 = t3 * t9258;
    let t83111 = t23452 * t6739 * t6741;
    (t83082, t83085, t83092, t83098, t83100, t83111)
}
