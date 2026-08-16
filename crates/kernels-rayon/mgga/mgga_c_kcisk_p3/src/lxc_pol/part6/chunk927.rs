//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 927/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk927(t196: f64, t28532: f64, t28696: f64, t673: f64, t11885: f64, t140: f64, t1470: f64, t18089: f64, t18132: f64, t2517: f64, t2521: f64, t28846: f64, t28851: f64, t28855: f64, t28897: f64, t28914: f64, t28918: f64, t29441: f64, t29445: f64, t29449: f64, t29453: f64, t29462: f64, t29466: f64, t479: f64, t5231: f64, t709: f64, t7349: f64, t7360: f64, t8919: f64, t8975: f64) -> f64 {
    let t29471 = t28532 * t196;
    let t29476 = t673 * t28696;
    let t29490 = -0.79593333333333333333e-1_f64 * t1470 * t29441 - 0.15918666666666666667e0_f64 * t1470 * t29445 - 0.79593333333333333333e-1_f64 * t1470 * t29449 - 0.26531111111111111111e-1_f64 * t1470 * t29453 + 0.17687407407407407407e-1_f64 * t18132 - t11885 - 0.27860625e-1_f64 * t5231 * t28851 - 0.27860625e-1_f64 * t5231 * t28846 + 0.26531111111111111111e0_f64 * t1470 * t29462 + 0.15918666666666666666e0_f64 * t1470 * t29466 + 0.5572125e-1_f64 * t5231 * t28855 + 0.619125e-2_f64 * t29471 * t709 + 0.371475e-1_f64 * t7360 * t28897 - 0.39796666666666666666e-1_f64 * t140 * t479 * t29476 - 0.139303125e-1_f64 * t7349 * t28914 + 0.139303125e-1_f64 * t7349 * t28918 - 0.5572125e-1_f64 * t18089 * t8919 + 0.27860625e-1_f64 * t8975 * t2517 - 0.1857375e-1_f64 * t8975 * t2521;
    t29490
}
