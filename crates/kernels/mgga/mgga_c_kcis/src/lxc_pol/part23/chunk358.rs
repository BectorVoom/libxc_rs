//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 358/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk358<F: Float>(t1616: F, t2109: F, t1592: F, t1620: F, t1949: F, t1985: F, t2004: F, t2008: F, t2014: F, t2093: F, t626: F, t2036: F, t2040: F, t2044: F, t2048: F, t2052: F, t2056: F, t2063: F, t2067: F) -> (F, F, F) {
    let t2110 = t2109 * t1616;
    let t2118 = t2093 * t626 - F::cast_from(0.66725e-1_f64) * t1592 * t2110 + t1620 + F::cast_from(0.11607361111111111111e-2_f64) * t1949 + F::cast_from(0.17411041666666666666e-2_f64) * t1985 - F::cast_from(0.17411041666666666666e-2_f64) * t2004 - F::cast_from(0.46429444444444444443e-2_f64) * t2008 + F::cast_from(0.11607361111111111111e-2_f64) * t2014;
    let t2128 = F::cast_from(0.9375e-1_f64) * t2036 - F::cast_from(0.9375e-1_f64) * t2040 - F::cast_from(0.25e0_f64) * t2044 + F::cast_from(0.625e-1_f64) * t2048 - F::cast_from(0.101171875e-1_f64) * t2052 + F::cast_from(0.101171875e-1_f64) * t2056 + F::cast_from(0.53958333333333333333e-1_f64) * t2063 - F::cast_from(0.13489583333333333333e-1_f64) * t2067;
    (t2110, t2118, t2128)
}
