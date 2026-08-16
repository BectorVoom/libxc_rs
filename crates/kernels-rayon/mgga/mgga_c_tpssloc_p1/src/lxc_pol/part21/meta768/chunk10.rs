//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2663/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2663(t5: f64, t55888: f64, t55924: f64, t112: f64, t4025: f64, t671: f64, t111: f64, t19449: f64, t2319: f64, t5449: f64, t1441: f64, t2363: f64, t2311: f64, t5456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t55926 = piecewise3(t8, 0.0_f64, t55888 + t55924);
    let t55927 = t55926 * t112;
    let t55934 = t4025 * t671;
    let t55943 = t19449 * t111;
    let t55946 = t5449 * t2319;
    let t55962 = t1441 * t2363;
    let t55967 = t2311 * t5456;
    (t55927, t55934, t55943, t55946, t55962, t55967)
}
