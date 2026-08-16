//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 771/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk771(t1922: f64, t979: f64, t4265: f64, t5256: f64, t5265: f64, t11334: f64, t11408: f64, t11413: f64, t11426: f64, t11439: f64, t11444: f64, t11815: f64, t11818: f64, t11823: f64, t11827: f64, t11830: f64, t11834: f64, t11838: f64, t11842: f64, t11851: f64, t1470: f64, t5231: f64, t6278: f64, t709: f64, t725: f64, t7349: f64, t7360: f64) -> f64 {
    let t11853 = t979 * t1922;
    let t11855 = t4265 * t5256;
    let t11857 = t4265 * t5265;
    let t11865 = 0.619125e-2_f64 * t11815 * t709 - 0.13265555555555555555e0_f64 * t6278 * t11818 + 0.15918666666666666666e0_f64 * t1470 * t11823 - 0.15918666666666666667e0_f64 * t1470 * t11827 - 0.88437037037037037035e-1_f64 * t11830 + 0.26531111111111111111e0_f64 * t1470 * t11834 - 0.79593333333333333333e-1_f64 * t1470 * t11838 - 0.26531111111111111111e-1_f64 * t1470 * t11842 - 0.139303125e-1_f64 * t7349 * t11439 + 0.139303125e-1_f64 * t7349 * t11334 + 0.5572125e-1_f64 * t5231 * t11413 + 0.10612444444444444444e0_f64 * t11851 + 0.17687407407407407407e-1_f64 * t11853 - 0.10612444444444444444e0_f64 * t11855 - 0.53062222222222222221e-1_f64 * t11857 - 0.27860625e-1_f64 * t5231 * t11426 + 0.371475e-1_f64 * t7360 * t11408 - 0.232171875e-2_f64 * t725 * t11444;
    t11865
}
