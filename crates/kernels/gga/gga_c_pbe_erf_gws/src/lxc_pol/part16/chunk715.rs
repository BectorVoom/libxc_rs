//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 715/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk715<F: Float>(t1662: F, t43: F, t1783: F, t636: F, t1841: F, t735: F, t155: F, t589: F, t592: F, t587: F, t1651: F, t1897: F, t1630: F, t1892: F, t639: F, t1641: F, t50: F) -> (F, F, F, F, F, F, F, F) {
    let t4957 = 1.0 / t1662 / t43;
    let t4985 = t1783 * t636;
    let t4987 = t1841 * t735;
    let t4991 = t155 * t589;
    let t4992 = t4991 * t592;
    let t4993 = t587 * t4992;
    let t4995 = t1651 * t1897;
    let t4996 = t587 * t4995;
    let t4998 = t1630 * t1892;
    let t4999 = t639 * t4998;
    let t5002 = 1.0 / t1641 / t50;
    (t4957, t4985, t4987, t4991, t4993, t4996, t4999, t5002)
}
