//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1363/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1363<F: Float>(t2864: F, t4545: F, t4558: F, t7819: F, t3799: F, t9722: F, t1115: F, t11761: F, t11766: F, t12029: F, t12041: F, t12046: F, t27882: F, t28165: F, t2901: F, t2910: F, t2923: F, t2953: F, t2957: F, t3724: F, t3784: F, t3788: F, t3823: F, t3826: F, t3829: F, t4619: F, t7849: F, t9737: F, t9758: F) -> (F, F, F) {
    let t33564 = t2864 * t4545;
    let t33572 = t4558 * t7819;
    let t33595 = t3799 * t9722;
    let t33598 = 800.0 / 27.0 * t12041 * t9737 - 3200.0 / 81.0 * t3823 * t33564 + 1600.0 / 27.0 * t12046 * t9737 - 0.384e-2 * t2953 * t4558 * t7849 - 0.5376e-2 * t2957 * t33572 - 0.176e-3 * t11766 * t2923 - 6400.0 / 27.0 * t3784 * t33564 - 6400.0 / 27.0 * t3788 * t33564 - 6400.0 / 81.0 * t3826 * t33564 - 6400.0 / 81.0 * t3829 * t33564 + 800.0 / 27.0 * t9758 * t12029 + 0.64e-1 * t11761 * t2910 - 800.0 / 27.0 * t3724 * t4545 * t1115 + 5040.0 * t27882 * t4619 * t2901 - 0.12096e2 * t28165 * t33595;
    (t33572, t33595, t33598)
}
