//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 581/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk581<F: Float>(t3330: F, t3444: F, t3453: F, t1010: F, t747: F, t106: F, t1046: F, t568: F, t933: F, t1146: F, t91: F, t97: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4245 = F::new(0.3056501876701794) * t3330;
    let t4252 = F::new(4.59690841536205) * t3444;
    let t4254 = F::new(12.258422440965466) * t3453;
    let t4274 = t747 * t1010;
    let t4275 = t106 * t4274;
    let t4277 = t568 * t1046;
    let t4278 = t933 * t4277;
    let t4280 = t91 * t1146;
    let t4281 = t4280 * t97;
    (t4245, t4252, t4254, t4274, t4275, t4277, t4278, t4280, t4281)
}
