//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1348/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1348<F: Float>(t10165: F, t5395: F, t4558: F, t7899: F, t10034: F, t10036: F, t10141: F, t10143: F, t10150: F, t10162: F, t11690: F, t11694: F, t11840: F, t1520: F, t28067: F, t28070: F, t28117: F, t28384: F, t28407: F, t28430: F, t28434: F, t32601: F, t32604: F, t32621: F, t32950: F, t32954: F, t3711: F, t3785: F, t3803: F) -> (F, F) {
    let t32978 = t5395 * t10165;
    let t33007 = t4558 * t7899;
    let t33010 = 0.26666666666666666666e0 * t32978 * t10143 - 0.74666666666666666666e1 * t28384 * t32601 + 0.26666666666666666666e0 * t28070 * t11694 - 0.26666666666666666666e0 * t28070 * t11690 - 0.37333333333333333334e1 * t10162 * t32950 - 0.10666666666666666667e1 * t28067 * t32601 + 0.53333333333333333334e1 * t28407 * t32604 - 0.53333333333333333334e0 * t10141 * t32950 + 0.26666666666666666667e1 * t10150 * t32954 + 400.0 / 9.0 * t1520 * t10034 * t10036 + 800.0 / 9.0 * t28117 * t11840 - 800.0 / 9.0 * t3803 * t3711 * t3785 + 0.36864e-7 * t28430 * t32621 - 0.36864e-7 * t28434 * t33007;
    (t33007, t33010)
}
