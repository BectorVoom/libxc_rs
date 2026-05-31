//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1119/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1119<F: Float>(t3390: F, t3469: F, t4927: F, t639: F, t1033: F, t12585: F, t32093: F, t1019: F, t12452: F, t18237: F, t34500: F, t43029: F, t47832: F, t47836: F, t47839: F, t47841: F, t47844: F) -> (F, F, F, F, F) {
    let t47848 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t639 * t4927 * t3469 * t3390;
    let t47850 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1033 * t12585;
    let t47851 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t32093;
    let t47855 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t12452 * t1019;
    let t47856 = -t47832 - t47836 + t47839 - t47841 - t47844 - t47848 - t47850 - t47851 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43029 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t34500 - t47855 + t18237;
    (t47848, t47850, t47851, t47855, t47856)
}
