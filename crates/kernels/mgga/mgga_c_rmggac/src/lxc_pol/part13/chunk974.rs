//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 974/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk974<F: Float>(t41438: F, t1652: F, t698: F, t2227: F, t551: F, t1614: F, t118: F, t2463: F, t27176: F, t326: F, t333: F, t352: F, t41116: F, t41458: F, t43698: F, t43854: F, t43981: F, t44157: F, t5148: F, t5155: F, t5245: F, t5266: F, t833: F, t848: F, t876: F, t9540: F, t9551: F) -> (F, F, F, F) {
    let t44169 = 0.3193131120497015617e0 * t41438;
    let t44183 = t698 * t1652;
    let t44187 = t2227 * t551;
    let t44194 = t698 * t1614;
    let t44203 = t44169 - 0.95793933614910468512e0 * t27176 * t43981 - 0.11974241701863808564e0 * t5148 * t9551 * t833 + 0.23948483403727617128e0 * t5266 * t44157 * t333 - 0.47896966807455234256e0 * t41116 * t9551 * t876 - 0.39914139006212695214e-1 * t118 * t43698 + 0.23948483403727617128e0 * t5266 * t44183 * t333 - 0.23948483403727617128e0 * t5148 * t44187 * t352 - 0.59871208509319042821e-1 * t326 * t43854 - 0.17961362552795712846e0 * t41458 + 0.47896966807455234256e0 * t5155 * t44194 * t333 + 0.23948483403727617128e0 * t5155 * t9540 * t848 + 0.11974241701863808564e0 * t5245 * t2463;
    (t44183, t44187, t44194, t44203)
}
