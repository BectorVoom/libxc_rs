//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 977/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk977<F: Float>(t11028: F, t1621: F, t639: F, t3478: F, t586: F, t645: F, t11004: F, t11009: F, t11014: F, t11016: F, t11018: F, t11021: F, t11024: F, t11027: F, t7852: F, t7870: F, t7873: F, t7876: F, t7880: F, t7890: F, t7905: F) -> (F, F, F) {
    let t11029 = t1621 * t11028;
    let t11031 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t639 * t11029;
    let t11032 = t3478 * t586;
    let t11034 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t11032 * t645;
    let t11035 = t11004 + t7852 + t7870 - t7873 - t7876 + t7880 + t7890 + t11009 + t11014 + t11016 - t7905 - t11018 + t11021 - t11024 - t11027 + t11031 + t11034;
    (t11031, t11034, t11035)
}
