//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1157/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1157<F: Float>(t1094: F, t16: F, t488: F, t8223: F, t2782: F, t7773: F, t2778: F, t7791: F, t2697: F, t7765: F, t7770: F, t1086: F, t1099: F, t2702: F, t7523: F, t2689: F, t7769: F) -> (F, F, F, F, F, F, F) {
    let t23984 = 0.18989649058080861537e-2 * t1094 * t16 * t8223 * t488;
    let t23985 = t7773 * t2782;
    let t23990 = t2778 * t7791;
    let t23992 = t2697 * t7765;
    let t23994 = t2697 * t7770;
    let t23999 = 0.46785788981077169656e1 * t1099 * t2702 * t7523 * t1086;
    let t24003 = 0.69263436422725855036e2 * t1099 * t2689 * t7523 * t7769;
    (t23984, t23985, t23990, t23992, t23994, t23999, t24003)
}
