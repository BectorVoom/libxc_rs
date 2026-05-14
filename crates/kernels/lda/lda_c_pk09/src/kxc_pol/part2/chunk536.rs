//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 536/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk536<F: Float>(t105: F, t4165: F, t3163: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F, t1010: F, t747: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4175 = t105 * t4165;
    let t4177 = t4175 * t3163 / 3.0;
    let t4187 = 0.9846956772543541 * t3397;
    let t4190 = 4.431130547644593 * t3409;
    let t4191 = 0.3928367389853144 * t3332;
    let t4192 = 0.06547278983088574 * t3339;
    let t4201 = 0.2946275542389858 * t3330;
    let t4208 = 4.431130547644593 * t3444;
    let t4210 = 11.81634812705225 * t3453;
    let t4231 = 1.0215352034137888 * t3397;
    let t4234 = 4.59690841536205 * t3409;
    let t4235 = 0.4075335835602392 * t3332;
    let t4236 = 0.06792226392670653 * t3339;
    let t4245 = 0.3056501876701794 * t3330;
    let t4252 = 4.59690841536205 * t3444;
    let t4254 = 12.258422440965466 * t3453;
    let t4274 = t747 * t1010;
    (t4177, t4187, t4190, t4191, t4192, t4201, t4208, t4210, t4231, t4234, t4235, t4236, t4245, t4252, t4254, t4274)
}
