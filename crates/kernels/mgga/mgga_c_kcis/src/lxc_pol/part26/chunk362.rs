//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 362/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk362<F: Float>(t1616: F, t2109: F, t1592: F, t1620: F, t1949: F, t1985: F, t2004: F, t2008: F, t2014: F, t2093: F, t626: F, t2036: F, t2040: F, t2044: F, t2048: F, t2052: F, t2056: F, t2063: F, t2067: F) -> (F, F, F) {
    let t2110 = t2109 * t1616;
    let t2118 = t2093 * t626 - F::new(0.66725e-1) * t1592 * t2110 + t1620 + F::new(0.11607361111111111111e-2) * t1949 + F::new(0.17411041666666666666e-2) * t1985 - F::new(0.17411041666666666666e-2) * t2004 - F::new(0.46429444444444444443e-2) * t2008 + F::new(0.11607361111111111111e-2) * t2014;
    let t2128 = F::new(0.9375e-1) * t2036 - F::new(0.9375e-1) * t2040 - F::new(0.25e0) * t2044 + F::new(0.625e-1) * t2048 - F::new(0.101171875e-1) * t2052 + F::new(0.101171875e-1) * t2056 + F::new(0.53958333333333333333e-1) * t2063 - F::new(0.13489583333333333333e-1) * t2067;
    (t2110, t2118, t2128)
}
