//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1310/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1310<F: Float>(t3174: F, t68: F, t8277: F, t1167: F, t6460: F, t8435: F, t8437: F, t926: F, t1228: F, t300: F, t2387: F, t919: F) -> (F, F, F, F, F) {
    let t23020 = t3174 * t68 * t8277;
    let t23022 = t1167 * t6460;
    let t23028 = t8435 * t926 * t8437;
    let t23054 = t300 * t1228;
    let t23055 = t2387 * t919;
    (t23020, t23022, t23028, t23054, t23055)
}
