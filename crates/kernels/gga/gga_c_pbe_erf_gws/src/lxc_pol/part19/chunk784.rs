//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 784/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk784<F: Float>(t5763: F, t5795: F, t119: F, t331: F, t481: F, t1557: F, t1513: F, t505: F, t96: F, t1235: F, t125: F, t128: F, t2: F) -> (F, F, F, F, F, F) {
    let t5796 = t5795 * t5763;
    let t5809 = t119 * t331 * t481;
    let t5810 = t1557 * t5809;
    let t5816 = t1513 * t5809;
    let t5825 = F::new(1.0) / t505 / t96;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    (t5796, t5810, t5816, t5825, t5832, t5833)
}
