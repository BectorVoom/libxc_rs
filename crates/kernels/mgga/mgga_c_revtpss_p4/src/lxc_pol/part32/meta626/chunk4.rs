//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1994/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1994<F: Float>(t1353: F, t2106: F, t101970: F, t28154: F, t101782: F, t101783: F, t101790: F, t101793: F, t101811: F, t101820: F, t108941: F, t1923: F, t2047: F, t28093: F, t28635: F, t30543: F, t6954: F, t7702: F, t7964: F, t95246: F) -> (F, F) {
    let t109874 = t1353 * t2106;
    let t109892 = t28154 * t101970;
    let t109895 = t101782 - F::cast_from(880.0_f64) / F::cast_from(27.0_f64) * t101783 + t101790 - F::cast_from(352.0_f64) / F::cast_from(27.0_f64) * t101793 + t101811 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28093 * t7964 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7702 * t28635 + t6954 * t30543 / F::cast_from(3.0_f64) + t1923 * t2047 * t108941 / F::cast_from(3.0_f64) - F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t109892 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t95246 + t101820;
    (t109874, t109895)
}
