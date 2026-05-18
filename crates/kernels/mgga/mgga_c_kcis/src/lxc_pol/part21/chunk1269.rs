//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1269/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1269<F: Float>(t3348: F, t4999: F, t14703: F, t26896: F, t26917: F, t28059: F, t1096: F, t14800: F, t8072: F, t92525: F, t14833: F, t92447: F) -> (F, F, F, F, F, F) {
    let t95444 = t4999 * t3348;
    let t95446 = t26896 * t14703;
    let t95448 = t28059 * t26917;
    let t95450 = t1096 * t14800;
    let t95453 = t92525 * t8072;
    let t95455 = t92447 * t14833;
    (t95444, t95446, t95448, t95450, t95453, t95455)
}
