//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 955/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk955<F: Float>(t2488: F, t4684: F, t7055: F, t6771: F, t708: F, t1648: F, t7028: F, t4652: F, t7029: F, t682: F, t1824: F, t4629: F, t7034: F, t4624: F, t7050: F, t4658: F, t6746: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16904 = t2488 * t4684;
    let t16905 = t7055 * t16904;
    let t16908 = t708 * t6771;
    let t16909 = t16908 * t1648;
    let t16910 = t7028 * t16909;
    let t16913 = t7029 * t4652;
    let t16914 = t7028 * t16913;
    let t16917 = t682 * t6771;
    let t16918 = t16917 * t1824;
    let t16919 = t4629 * t16918;
    let t16922 = t7034 * t4684;
    let t16923 = t4629 * t16922;
    let t16926 = t7050 * t4624;
    let t16927 = t7028 * t16926;
    let t16930 = t6746 * t4658;
    (t16904, t16905, t16909, t16910, t16913, t16914, t16918, t16919, t16922, t16923, t16926, t16927, t16930)
}
