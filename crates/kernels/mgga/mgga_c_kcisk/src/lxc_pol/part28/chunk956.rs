//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 956/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk956<F: Float>(t1417: F, t8932: F, t1060: F, t22434: F, t16960: F, t16845: F, t16839: F, t7046: F, t8924: F, t6771: F, t7029: F, t7028: F, t16917: F, t2487: F, t4629: F, t6790: F, t7034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22512 = t1417 * t8932;
    let t22514 = t22434 * t1060;
    let t22515 = t16960 * t22514;
    let t22518 = t16845 * t22514;
    let t22521 = t16839 * t7046;
    let t22524 = t1417 * t8924;
    let t22526 = t7029 * t6771;
    let t22527 = t7028 * t22526;
    let t22530 = t16917 * t2487;
    let t22531 = t4629 * t22530;
    let t22534 = t7034 * t6790;
    (t22512, t22514, t22515, t22518, t22521, t22524, t22526, t22527, t22530, t22531, t22534)
}
