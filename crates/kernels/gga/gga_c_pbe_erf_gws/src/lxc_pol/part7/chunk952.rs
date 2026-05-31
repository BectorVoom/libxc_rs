//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 952/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk952<F: Float>(t1640: F, t1791: F, t1413: F, t1642: F, t1793: F, t639: F, t1620: F, t4934: F, t5141: F, t5155: F, t7877: F, t17001: F, t2677: F) -> (F, F, F, F) {
    let t17646 = t1640 * t1791;
    let t17651 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t639 * t17646 * t1793 * t1642 * t1413;
    let t17653 = t1620 * t4934 * t5141;
    let t17654 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17653;
    let t17656 = t1620 * t7877 * t5155;
    let t17657 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t17656;
    let t17660 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t639 * t2677 * t17001;
    (t17651, t17654, t17657, t17660)
}
