//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 831/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk831<F: Float>(t11885: F, t140: F, t1470: F, t18089: F, t18132: F, t2517: F, t2521: F, t28846: F, t28851: F, t28855: F, t28897: F, t28914: F, t28918: F, t29441: F, t29445: F, t29449: F, t29453: F, t29462: F, t29466: F, t29471: F, t29476: F, t479: F, t5231: F, t709: F, t7349: F, t7360: F, t8919: F, t8975: F) -> (F,) {
    let t29490 = -0.79593333333333333333e-1 * t1470 * t29441 - 0.15918666666666666667e0 * t1470 * t29445 - 0.79593333333333333333e-1 * t1470 * t29449 - 0.26531111111111111111e-1 * t1470 * t29453 + 0.17687407407407407407e-1 * t18132 - t11885 - 0.27860625e-1 * t5231 * t28851 - 0.27860625e-1 * t5231 * t28846 + 0.26531111111111111111e0 * t1470 * t29462 + 0.15918666666666666666e0 * t1470 * t29466 + 0.5572125e-1 * t5231 * t28855 + 0.619125e-2 * t29471 * t709 + 0.371475e-1 * t7360 * t28897 - 0.39796666666666666666e-1 * t140 * t479 * t29476 - 0.139303125e-1 * t7349 * t28914 + 0.139303125e-1 * t7349 * t28918 - 0.5572125e-1 * t18089 * t8919 + 0.27860625e-1 * t8975 * t2517 - 0.1857375e-1 * t8975 * t2521;
    (t29490,)
}
