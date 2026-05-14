//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1038/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1038<F: Float>(t27: F, t82: F, t2489: F, t4805: F, t16111: F, t4795: F, t973: F, t1424: F, t1429: F, t440: F, t2493: F, t500: F, t7: F, t16089: F, t4811: F, t983: F) -> (F, F, F, F, F, F, F) {
    let t19418 = t27 * t82;
    let t19427 = t2489 * t4805;
    let t19435 = t16111 * t973 * t4795;
    let t19439 = t1424 * t1429 * t440;
    let t19442 = t2493 * t500;
    let t19444 = 20.0 * t7 * t19442;
    let t19446 = t16089 * t983 * t4811;
    (t19418, t19427, t19435, t19439, t19442, t19444, t19446)
}
