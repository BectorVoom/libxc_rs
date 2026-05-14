//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1134/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1134<F: Float>(t1141: F, t29024: F, t1203: F, t10498: F, t5189: F, t8064: F, t20210: F, t7740: F, t27999: F, t46026: F, t63371: F, t7743: F, t3330: F, t8081: F, t26868: F, t6735: F) -> (F, F, F, F, F, F, F) {
    let t100929 = t29024 * t1141;
    let t100930 = t100929 * t1203;
    let t100933 = 12.0 * t10498 * t8064 * t5189;
    let t100936 = t7740 * t20210;
    let t100940 = 12.0 * t46026 * t27999;
    let t100942 = 2.0 * t63371 * t7743;
    let t100945 = 4.0 * t3330 * t8081 * t5189;
    let t100950 = t26868 * t6735;
    (t100930, t100933, t100936, t100940, t100942, t100945, t100950)
}
