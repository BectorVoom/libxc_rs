//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1059/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1059<F: Float>(t14409: F, t1470: F, t21152: F, t2221: F, t2225: F, t2242: F, t27270: F, t31089: F, t31093: F, t31100: F, t31324: F, t31328: F, t31332: F, t31336: F, t31339: F, t31343: F, t31347: F, t460: F, t476: F, t7865: F, t7873: F, t7898: F, t8192: F) -> F {
    let t31350 = F::new(0.9286875e-2) * t476 * t31089 - F::new(0.619125e-2) * t476 * t31100 + F::cast_from(0.139303125e-1_f64) * t2242 * t7865 - t14409 + F::cast_from(0.17687407407407407407e-1_f64) * t21152 + F::cast_from(0.10612444444444444444e0_f64) * t27270 + F::new(0.27860625e-1) * t8192 * t2221 - F::new(0.1857375e-1) * t8192 * t2225 + F::new(0.27860625e-1) * t2242 * t7873 - F::new(0.1857375e-1) * t2242 * t7898 - F::new(0.371475e-1) * t476 * t31093 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t31324 - F::cast_from(0.15918666666666666667e0_f64) * t1470 * t31328 + F::cast_from(0.26531111111111111111e0_f64) * t1470 * t31332 + F::cast_from(0.15918666666666666666e0_f64) * t1470 * t31336 + F::new(0.619125e-2) * t31339 * t460 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t31343 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t31347;
    t31350
}
