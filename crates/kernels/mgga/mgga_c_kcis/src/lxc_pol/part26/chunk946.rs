//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 946/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk946<F: Float>(t21922: F, t4160: F, t17292: F, t5649: F, t5655: F, t20974: F, t5662: F, t4162: F, t5661: F, t11854: F, t20979: F, t4170: F) -> (F, F, F, F, F, F, F) {
    let t21923 = t4160 * t21922;
    let t21925 = t17292 * t5649;
    let t21926 = t4160 * t21925;
    let t21928 = t17292 * t5655;
    let t21929 = t4160 * t21928;
    let t21931 = t5662 * t20974;
    let t21932 = t4162 * t21931;
    let t21933 = t5661 * t21932;
    let t21935 = t11854 * t20979;
    let t21936 = t4170 * t21935;
    (t21923, t21926, t21929, t21931, t21933, t21935, t21936)
}
