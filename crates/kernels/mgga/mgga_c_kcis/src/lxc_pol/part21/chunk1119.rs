//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1119/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1119<F: Float>(t26917: F, t28059: F, t1096: F, t14800: F, t8072: F, t92525: F, t14833: F, t92447: F, t14686: F, t26930: F, t14718: F, t28029: F, t3432: F, t5026: F, t3463: F, t376: F) -> (F, F, F, F, F, F, F, F) {
    let t95448 = t28059 * t26917;
    let t95450 = t1096 * t14800;
    let t95453 = t92525 * t8072;
    let t95455 = t92447 * t14833;
    let t95457 = t26930 * t14686;
    let t95459 = t28029 * t14718;
    let t95461 = t5026 * t3432;
    let t95463 = t3463 * t376;
    (t95448, t95450, t95453, t95455, t95457, t95459, t95461, t95463)
}
