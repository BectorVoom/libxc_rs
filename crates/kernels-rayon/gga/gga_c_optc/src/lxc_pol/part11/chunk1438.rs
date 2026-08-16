//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1438/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1438(t3107: f64, t58941: f64, t1150: f64, t11937: f64, t123: f64, t12567: f64, t12617: f64, t17920: f64, t18085: f64, t27438: f64, t27441: f64, t3212: f64, t36182: f64, t429: f64, t438: f64, t45731: f64, t45788: f64, t458: f64, t55332: f64, t55341: f64, t55343: f64, t55346: f64, t55361: f64, t55364: f64, t55487: f64, t59434: f64, t59855: f64, t914: f64) -> (f64, f64) {
    let t60009 = t58941 * t3107;
    let t60031 = 0.7727254657590006982e-1_f64 * t45731 + 0.11360101276506094136e1_f64 * t1150 * t914 * t429 * t59434 * t438 + 0.26372962023724310886e4_f64 * t3212 * t458 * t60009 * t123 - 0.90880810212048753088e1_f64 * t12567 * t12617 * t59855 + 0.31957282085435444036e5_f64 * t27438 * t55487 * t27441 * t17920 - 0.45352564237957702055e6_f64 * t55332 + 0.75587607063262836759e5_f64 * t55341 + 0.80609127133382715661e-1_f64 * t55343 + 0.23181763972770020946e0_f64 * t55346 + 0.45352564237957702055e6_f64 * t55361 - 0.61944912485988186948e2_f64 * t55364 + 0.49555929988790549556e3_f64 * t11937 * t18085 - 0.47768371634597164836e-1_f64 * t36182 + 0.71652557451895747254e-1_f64 * t45788;
    (t60009, t60031)
}
