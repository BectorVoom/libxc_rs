//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1405/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1405<F: Float>(t20160: F, t33836: F, t9536: F, t109514: F, t33915: F, t33910: F, t32439: F, t109399: F, t109963: F, t113584: F, t114978: F, t114982: F, t114983: F, t114991: F, t2736: F, t2740: F, t32354: F, t32443: F, t32458: F, t33794: F, t33906: F, t3575: F, t3583: F, t55401: F, t79: F, t9867: F) -> (F, F, F) {
    let t114995 = t20160 * t33836;
    let t114997 = 0.34722222222222222222e-2 * t9536 * t114995;
    let t115001 = 0.23148148148148148148e-2 * t9536 * t109514 * t33915;
    let t115002 = t109514 * t33910;
    let t115004 = 0.44675925925925925926e-3 * t32439 * t115002;
    let t115015 = -0.20104166666666666667e-2 * t32439 * t114978 - t114982 + 0.11574074074074074074e-2 * t114983 - 0.52083333333333333333e-2 * t55401 * t79 * t2736 * t2740 - 0.13402777777777777778e-2 * t109399 - 0.69444444444444444445e-2 * t114991 - 0.10416666666666666667e-1 * t33794 * t32443 - t114997 - 0.41270617283950617284e-2 * t113584 + t115001 + t115004 + 0.17361111111111111111e-2 * t9536 * t32458 * t9867 * t3583 + 0.23148148148148148148e-2 * t9536 * t109963 * t9867 * t3575 + 0.34722222222222222222e-2 * t32354 * t33906;
    (t114995, t115002, t115015)
}
