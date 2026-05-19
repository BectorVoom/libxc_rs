//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 771/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk771<F: Float>(t1922: F, t979: F, t4265: F, t5256: F, t5265: F, t11334: F, t11408: F, t11413: F, t11426: F, t11439: F, t11444: F, t11815: F, t11818: F, t11823: F, t11827: F, t11830: F, t11834: F, t11838: F, t11842: F, t11851: F, t1470: F, t5231: F, t6278: F, t709: F, t725: F, t7349: F, t7360: F) -> F {
    let t11853 = t979 * t1922;
    let t11855 = t4265 * t5256;
    let t11857 = t4265 * t5265;
    let t11865 = F::new(0.619125e-2) * t11815 * t709 - F::cast_from(0.13265555555555555555e0_f64) * t6278 * t11818 + F::cast_from(0.15918666666666666666e0_f64) * t1470 * t11823 - F::cast_from(0.15918666666666666667e0_f64) * t1470 * t11827 - F::cast_from(0.88437037037037037035e-1_f64) * t11830 + F::cast_from(0.26531111111111111111e0_f64) * t1470 * t11834 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t11838 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t11842 - F::cast_from(0.139303125e-1_f64) * t7349 * t11439 + F::cast_from(0.139303125e-1_f64) * t7349 * t11334 + F::new(0.5572125e-1) * t5231 * t11413 + F::cast_from(0.10612444444444444444e0_f64) * t11851 + F::cast_from(0.17687407407407407407e-1_f64) * t11853 - F::cast_from(0.10612444444444444444e0_f64) * t11855 - F::cast_from(0.53062222222222222221e-1_f64) * t11857 - F::new(0.27860625e-1) * t5231 * t11426 + F::new(0.371475e-1) * t7360 * t11408 - F::cast_from(0.232171875e-2_f64) * t725 * t11444;
    t11865
}
