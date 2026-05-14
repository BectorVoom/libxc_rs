//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1153/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1153<F: Float>(t2634: F, t2659: F, t2665: F, t2633: F, t7583: F, t7586: F, t1035: F, t2667: F, t7577: F, t2668: F, t7607: F, t23864: F, t2741: F, t23671: F, t2630: F, t2663: F, t437: F) -> (F, F, F, F, F, F, F, F) {
    let t23874 = 36.0 * t2665 * t2634 * t2659;
    let t23878 = 0.3103560775156404018e4 * t7583 * t2633 * t7586 * t2659;
    let t23882 = 0.64327917994770140268e2 * t2665 * t7577 * t2667 * t1035;
    let t23885 = 0.57895126195293126241e3 * t7607 * t2668 * t2659;
    let t23888 = 0.57895126195293126241e3 * t7583 * t23864 * t2667;
    let t23892 = t2741 * t2741;
    let t23904 = 0.48245938496077605201e2 * t2665 * t23671 * t2667;
    let t23910 = 0.62071215503128080361e4 * t437 / t2663 / t2630 * t23864 * t7586;
    (t23874, t23878, t23882, t23885, t23888, t23892, t23904, t23910)
}
