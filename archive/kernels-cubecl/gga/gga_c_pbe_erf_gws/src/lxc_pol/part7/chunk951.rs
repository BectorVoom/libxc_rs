//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 951/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk951<F: Float>(t1765: F, t1827: F, t1866: F, t587: F, t4958: F, t5543: F, t610: F, t1620: F, t4902: F, t4934: F, t1416: F, t4927: F, t4928: F, t639: F) -> (F, F, F, F) {
    let t17634 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t587 * t1827 * t1765 * t1866;
    let t17638 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t587 * t5543 * t4958 * t610;
    let t17640 = t1620 * t4934 * t4902;
    let t17641 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17640;
    let t17645 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t639 * t4927 * t4928 * t1416;
    (t17634, t17638, t17641, t17645)
}
