//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 934/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk934<F: Float>(t196: F, t30738: F, t1471: F, t6298: F, t7710: F, t1472: F, t30158: F, t14409: F, t1470: F, t21152: F, t2221: F, t2225: F, t2242: F, t27270: F, t31089: F, t31093: F, t31100: F, t31324: F, t31328: F, t31332: F, t31336: F, t460: F, t476: F, t7865: F, t7873: F, t7898: F, t8192: F) -> (F,) {
    let t31339 = t30738 * t196;
    let t31343 = t1471 * t6298 * t7710;
    let t31347 = t1471 * t1472 * t30158;
    let t31350 = 0.9286875e-2 * t476 * t31089 - 0.619125e-2 * t476 * t31100 + 0.139303125e-1 * t2242 * t7865 - t14409 + 0.17687407407407407407e-1 * t21152 + 0.10612444444444444444e0 * t27270 + 0.27860625e-1 * t8192 * t2221 - 0.1857375e-1 * t8192 * t2225 + 0.27860625e-1 * t2242 * t7873 - 0.1857375e-1 * t2242 * t7898 - 0.371475e-1 * t476 * t31093 - 0.79593333333333333333e-1 * t1470 * t31324 - 0.15918666666666666667e0 * t1470 * t31328 + 0.26531111111111111111e0 * t1470 * t31332 + 0.15918666666666666666e0 * t1470 * t31336 + 0.619125e-2 * t31339 * t460 - 0.79593333333333333333e-1 * t1470 * t31343 - 0.26531111111111111111e-1 * t1470 * t31347;
    (t31350,)
}
