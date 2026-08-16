//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1059/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1059(t14409: f64, t1470: f64, t21152: f64, t2221: f64, t2225: f64, t2242: f64, t27270: f64, t31089: f64, t31093: f64, t31100: f64, t31324: f64, t31328: f64, t31332: f64, t31336: f64, t31339: f64, t31343: f64, t31347: f64, t460: f64, t476: f64, t7865: f64, t7873: f64, t7898: f64, t8192: f64) -> f64 {
    let t31350 = 0.9286875e-2_f64 * t476 * t31089 - 0.619125e-2_f64 * t476 * t31100 + 0.139303125e-1_f64 * t2242 * t7865 - t14409 + 0.17687407407407407407e-1_f64 * t21152 + 0.10612444444444444444e0_f64 * t27270 + 0.27860625e-1_f64 * t8192 * t2221 - 0.1857375e-1_f64 * t8192 * t2225 + 0.27860625e-1_f64 * t2242 * t7873 - 0.1857375e-1_f64 * t2242 * t7898 - 0.371475e-1_f64 * t476 * t31093 - 0.79593333333333333333e-1_f64 * t1470 * t31324 - 0.15918666666666666667e0_f64 * t1470 * t31328 + 0.26531111111111111111e0_f64 * t1470 * t31332 + 0.15918666666666666666e0_f64 * t1470 * t31336 + 0.619125e-2_f64 * t31339 * t460 - 0.79593333333333333333e-1_f64 * t1470 * t31343 - 0.26531111111111111111e-1_f64 * t1470 * t31347;
    t31350
}
