//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 953/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk953<F: Float>(t12339: F, t1820: F, t1821: F, t7899: F, t3425: F, t3454: F, t5548: F, t587: F, t1017: F, t12472: F, t1827: F, t12717: F, t2615: F, t12767: F, t7527: F, t1620: F, t1809: F, t41459: F, t954: F) -> (F, F, F, F, F, F) {
    let t47315 = 64.0 / 15.0 * t1820 * t1821 * t7899 * t12339;
    let t47319 = 32.0 / 15.0 * t587 * t5548 * t3425 * t3454;
    let t47323 = 32.0 / 15.0 * t587 * t1827 * t12472 * t1017;
    let t47325 = 16.0 / 15.0 * t2615 * t12717;
    let t47327 = 32.0 / 15.0 * t7527 * t12767;
    let t47331 = 32.0 / 45.0 * t1620 * t1809 * t41459 * t954;
    (t47315, t47319, t47323, t47325, t47327, t47331)
}
