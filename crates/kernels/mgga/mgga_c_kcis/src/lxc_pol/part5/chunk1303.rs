//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1303/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1303<F: Float>(t4142: F, t7034: F, t11814: F, t7038: F, t3728: F, t6933: F, t7113: F, t833: F, t1409: F, t6281: F, t1419: F, t167: F, t1951: F) -> (F, F, F, F, F, F) {
    let t21518 = t4142 * t7034;
    let t21520 = t11814 * t7038;
    let t21522 = t3728 * t6933;
    let t21524 = t7113 * t833;
    let t21527 = t1409 * t6281;
    let t21528 = t21527 * t1419;
    let t21531 = t1951 * t167;
    (t21518, t21520, t21522, t21524, t21528, t21531)
}
