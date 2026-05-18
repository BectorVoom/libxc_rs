//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1301/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1301<F: Float>(t102079: F, t5426: F, t99208: F, t2109: F, t531: F, t28753: F, t99320: F, t1307: F, t29524: F, t95024: F, t1464: F, t22259: F, t28503: F) -> (F, F, F, F, F) {
    let t102275 = t99208 * t5426 * t102079;
    let t102278 = t2109 * t531;
    let t102280 = t99320 * t102278 * t28753;
    let t102286 = t95024 * t29524 * t1307;
    let t102292 = t1464 * t28503 * t22259;
    (t102275, t102278, t102280, t102286, t102292)
}
