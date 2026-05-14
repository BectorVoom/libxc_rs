//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1344/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1344<F: Float>(t118802: F, t9426: F, t26905: F, t32122: F, t415: F, t34777: F, t9442: F, t2236: F, t5868: F, t110762: F, t114231: F, t114712: F, t114714: F, t114716: F, t114728: F, t114738: F, t114773: F, t114774: F, t2718: F, t33481: F, t6221: F, t9796: F) -> (F, F, F) {
    let t119632 = t9426 * t118802;
    let t119636 = t415 * t32122 * t26905;
    let t119642 = t34777 * t9442;
    let t119645 = t415 * t5868 * t2236;
    let t119647 = 0.20833333333333333334e-1 * t114231 * t9796 + 0.13402777777777777778e-2 * t119632 + t114712 + t114714 - t114716 - t114728 - 0.58958024691358024688e-2 * t114738 + t110762 - 0.13265555555555555555e-1 * t119636 + t114773 - 0.88437037037037037035e-2 * t114774 + 0.55555555555555555557e-1 * t6221 * t33481 * t2718 - 0.69444444444444444447e-2 * t119642 + 0.33163888888888888888e-2 * t119645;
    (t119636, t119645, t119647)
}
