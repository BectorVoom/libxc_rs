//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 938/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk938<F: Float>(t1173: F, t7894: F, t13009: F, t7757: F, t1175: F, t5684: F, t5690: F, t3598: F, t7764: F, t25620: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25607: F, t25609: F, t25612: F, t25615: F, t25618: F) -> (F, F, F, F, F, F) {
    let t25623 = t1173 * t7894;
    let t25626 = t13009 * t7757;
    let t25627 = t25626 * t1175;
    let t25629 = t5690 * t5684;
    let t25631 = t3598 * t7764;
    let t25632 = t25631 * t1175;
    let t25634 = t1173 * t25620;
    let t25652 = 0.91722222222222222223e-3 * t25590 - 0.45861111111111111112e-2 * t25593 + 0.1651e-1 * t25596 - 0.11006666666666666667e-1 * t25599 - 0.27516666666666666667e-2 * t25601 - 0.24765e-1 * t25604 + 0.3302e-1 * t25607 + 0.13758333333333333333e-2 * t25609 - 0.27516666666666666667e-2 * t25612 + 0.8255e-2 * t25615 - 0.41275e-2 * t25618;
    (t25623, t25627, t25629, t25632, t25634, t25652)
}
