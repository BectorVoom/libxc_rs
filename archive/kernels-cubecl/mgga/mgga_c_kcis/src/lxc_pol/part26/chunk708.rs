//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 708/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk708<F: Float>(t7952: F, t7953: F, t3738: F, t585: F, t1468: F, t1548: F, t1395: F, t1552: F, t7946: F, t7950: F) -> (F, F, F, F, F) {
    let t7954 = t7952 * t7953;
    let t7956 = t3738 * t585;
    let t7958 = t1468 * t1548;
    let t7960 = t1395 * t1552;
    let t7962 = t7946 / F::cast_from(16.0_f64) - t7950 / F::cast_from(16.0_f64) + t7954 / F::cast_from(24.0_f64) - t7956 / F::cast_from(128.0_f64) + t7958 / F::cast_from(128.0_f64) - t7960 / F::cast_from(96.0_f64);
    (t7954, t7956, t7958, t7960, t7962)
}
