//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 826/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk826<F: Float>(t1014: F, t5891: F, t3728: F, t5629: F, t14249: F, t5446: F, t3255: F, t5460: F, t5465: F, t11633: F, t1897: F, t518: F, t5481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15986 = t1014 * t5891;
    let t15987 = F::cast_from(0.88437037037037037034e-2_f64) * t15986;
    let t15988 = t3728 * t5629;
    let t15989 = F::cast_from(0.33163888888888888888e-2_f64) * t15988;
    let t15994 = t14249 * t5446;
    let t16001 = F::cast_from(0.98556445e-3_f64) * t3255 * t5460;
    let t16003 = F::cast_from(0.19711289e-2_f64) * t3255 * t5465;
    let t16025 = t11633 * t1897;
    let t16029 = t518 * t5481;
    (t15986, t15987, t15988, t15989, t15994, t16001, t16003, t16025, t16029)
}
