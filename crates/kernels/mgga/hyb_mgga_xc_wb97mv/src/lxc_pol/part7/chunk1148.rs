//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1148/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1148<F: Float>(t1099: F, t23583: F, t23664: F, t7535: F, t1041: F, t7543: F, t2659: F, t1036: F, t2632: F, t7737: F, t7749: F, t1014: F, t2626: F, t7525: F, t7746: F, t7536: F) -> (F, F, F, F, F, F, F, F) {
    let t23668 = 0.12304822629859687989e5 * t1099 * t23664 * t23583 * t7535;
    let t23669 = t1041 * t7543;
    let t23671 = t2659 * t2659;
    let t23674 = 6.0 * t2632 * t23671 * t1036;
    let t23675 = t7737 * t7749;
    let t23679 = 0.21687162600603479684e-1 * t2626 * t1014 * t7525;
    let t23680 = t7737 * t7746;
    let t23684 = 0.38025319932552508021e2 * t2626 * t1014 * t7536;
    (t23668, t23669, t23671, t23674, t23675, t23679, t23680, t23684)
}
