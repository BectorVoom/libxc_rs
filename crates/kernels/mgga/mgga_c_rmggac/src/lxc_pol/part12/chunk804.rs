//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 804/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk804<F: Float>(t2286: F, t35384: F, t1175: F, t1971: F, t511: F, t558: F, t8517: F, t34884: F, t9206: F, t2295: F, t27006: F, t1475: F, t1970: F, t848: F, t515: F, t866: F) -> (F, F, F, F, F, F) {
    let t39584 = t35384 * t2286;
    let t39589 = t8517 * t1971 * t511 * t558 * t1175;
    let t39591 = t34884 * t9206;
    let t39595 = t27006 * t2295;
    let t39600 = t1970 * t1971 * t511 * t1475 * t848;
    let t39605 = t1970 * t1971 * t515 * t1475 * t866;
    (t39584, t39589, t39591, t39595, t39600, t39605)
}
