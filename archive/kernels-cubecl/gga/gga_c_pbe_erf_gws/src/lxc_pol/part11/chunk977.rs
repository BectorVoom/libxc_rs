//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 977/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk977<F: Float>(t1778: F, t3493: F, t17470: F, t1820: F, t3534: F, t2615: F, t7579: F, t3526: F, t4991: F, t587: F, t1986: F, t3459: F) -> (F, F, F, F, F) {
    let t32670 = t3493 * t1778;
    let t32704 = t1820 * t17470 * t3534;
    let t32710 = t2615 * t7579;
    let t32739 = t587 * t4991 * t3526;
    let t32759 = t3459 * t1986;
    (t32670, t32704, t32710, t32739, t32759)
}
