//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1135/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1135<F: Float>(t27836: F, t7719: F, t1020: F, t26753: F, t8047: F, t167: F, t3203: F, t7718: F, t4994: F, t1014: F, t8057: F, t356: F, t5013: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27837 = t27836 * t7719;
    let t27838 = t1020 * t27837;
    let t27842 = t26753 * t8047;
    let t27843 = t1020 * t27842;
    let t27845 = t3203 * t167;
    let t27846 = t7718 * t27845;
    let t27847 = t4994 * t27846;
    let t27849 = t1014 * t8057;
    let t27851 = t356 * t5013;
    (t27837, t27838, t27842, t27843, t27845, t27846, t27847, t27849, t27851)
}
