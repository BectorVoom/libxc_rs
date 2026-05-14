//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1155/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1155<F: Float>(t2745: F, t2748: F, t1070: F, t1079: F, t1085: F, t221: F, t23583: F, t23616: F, t23633: F, t23639: F, t23642: F, t23664: F, t23697: F, t23874: F, t23878: F, t23882: F, t23885: F, t23888: F, t23892: F, t23904: F, t23910: F, t23924: F, t23932: F, t2730: F, t2747: F, t2749: F, t466: F, t475: F, t479: F, t488: F, t7535: F, t7620: F) -> (F,) {
    let t23939 = t2745 * t2745;
    let t23942 = t2748 * t2748;
    let t23955 = -t23874 - t23878 - t23882 + t23885 - t23888 - 0.55209406483950617283e-2 * t221 * t23616 * t475 - 6.0 * t2730 * t23892 * t1070 - 24.0 * t7620 * t23697 * t1070 - 0.18989649058080861537e-2 * t221 * t23616 * t488 - t23904 + t23910 - t23924 - t23932 + 0.96491876992155210402e2 * t2747 * t23892 * t2749 + 0.5848223622634646207e0 * t1079 * t23633 * t1085 + 0.19964560303604640732e6 * t466 / t23939 * t23697 / t23942 + 0.91082604192152556044e5 * t479 * t23639 * t23583 * t23642 - 0.12304822629859687989e5 * t479 * t23664 * t23583 * t7535;
    (t23955,)
}
