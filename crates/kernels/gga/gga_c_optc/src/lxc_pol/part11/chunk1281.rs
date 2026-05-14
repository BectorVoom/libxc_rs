//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1281/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1281<F: Float>(t3107: F, t58941: F, t1150: F, t11937: F, t123: F, t12567: F, t12617: F, t17920: F, t18085: F, t27438: F, t27441: F, t3212: F, t36182: F, t429: F, t438: F, t45731: F, t45788: F, t458: F, t55332: F, t55341: F, t55343: F, t55346: F, t55361: F, t55364: F, t55487: F, t59434: F, t59855: F, t914: F) -> (F, F) {
    let t60009 = t58941 * t3107;
    let t60031 = 0.7727254657590006982e-1 * t45731 + 0.11360101276506094136e1 * t1150 * t914 * t429 * t59434 * t438 + 0.26372962023724310886e4 * t3212 * t458 * t60009 * t123 - 0.90880810212048753088e1 * t12567 * t12617 * t59855 + 0.31957282085435444036e5 * t27438 * t55487 * t27441 * t17920 - 0.45352564237957702055e6 * t55332 + 0.75587607063262836759e5 * t55341 + 0.80609127133382715661e-1 * t55343 + 0.23181763972770020946e0 * t55346 + 0.45352564237957702055e6 * t55361 - 0.61944912485988186948e2 * t55364 + 0.49555929988790549556e3 * t11937 * t18085 - 0.47768371634597164836e-1 * t36182 + 0.71652557451895747254e-1 * t45788;
    (t60009, t60031)
}
