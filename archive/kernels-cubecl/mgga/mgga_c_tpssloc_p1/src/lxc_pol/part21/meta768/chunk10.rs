//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2663/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2663<F: Float>(t5: F, t55888: F, t55924: F, t112: F, t4025: F, t671: F, t111: F, t19449: F, t2319: F, t5449: F, t1441: F, t2363: F, t2311: F, t5456: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t55926 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t55888 + t55924);
    let t55927 = t55926 * t112;
    let t55934 = t4025 * t671;
    let t55943 = t19449 * t111;
    let t55946 = t5449 * t2319;
    let t55962 = t1441 * t2363;
    let t55967 = t2311 * t5456;
    (t55927, t55934, t55943, t55946, t55962, t55967)
}
