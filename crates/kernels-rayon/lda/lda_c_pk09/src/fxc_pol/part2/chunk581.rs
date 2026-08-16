//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 581/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk581(t3330: f64, t3444: f64, t3453: f64, t1010: f64, t747: f64, t106: f64, t1046: f64, t568: f64, t933: f64, t1146: f64, t91: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4245 = 0.3056501876701794_f64 * t3330;
    let t4252 = 4.59690841536205_f64 * t3444;
    let t4254 = 12.258422440965466_f64 * t3453;
    let t4274 = t747 * t1010;
    let t4275 = t106 * t4274;
    let t4277 = t568 * t1046;
    let t4278 = t933 * t4277;
    let t4280 = t91 * t1146;
    let t4281 = t4280 * t97;
    (t4245, t4252, t4254, t4274, t4275, t4277, t4278, t4280, t4281)
}
