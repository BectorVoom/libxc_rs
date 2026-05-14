//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1183/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1183<F: Float>(t21655: F, t5653: F, t4170: F, t16771: F, t1307: F, t7309: F, t4160: F, t1459: F, t7104: F, t303: F, t1489: F, t6922: F, t4135: F, t1468: F, t1464: F, t1497: F) -> (F, F, F, F, F, F) {
    let t21894 = t5653 * t21655;
    let t21895 = t4170 * t21894;
    let t21896 = t16771 * t21895;
    let t21898 = t7309 * t1307;
    let t21899 = t4170 * t21898;
    let t21900 = t4160 * t21899;
    let t21902 = t1459 * t7104;
    let t21903 = t303 * t21902;
    let t21905 = t6922 * t1489;
    let t21906 = t4135 * t21905;
    let t21907 = t1468 * t21906;
    let t21908 = t1464 * t21907;
    let t21910 = t6922 * t1497;
    (t21896, t21900, t21903, t21905, t21908, t21910)
}
