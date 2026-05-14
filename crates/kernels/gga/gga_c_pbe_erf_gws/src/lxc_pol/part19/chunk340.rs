//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 340/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk340<F: Float>(t1000: F, t571: F, t11: F, t570: F, t173: F, t184: F) -> (F, F, F, F, F) {
    let t1001 = t571 * t1000;
    let t1002 = t11 * t1001;
    let t1004 = t570 + 0.18891666666666666667e-2 * t1002;
    let t1005 = t173 * t1004;
    let t1006 = t1005 * t184;
    (t1001, t1002, t1004, t1005, t1006)
}
