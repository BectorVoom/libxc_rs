//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 902/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk902(t2286: f64, t35384: f64, t1175: f64, t1971: f64, t511: f64, t558: f64, t8517: f64, t34884: f64, t9206: f64, t2295: f64, t27006: f64, t1475: f64, t1970: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t39584 = t35384 * t2286;
    let t39589 = t8517 * t1971 * t511 * t558 * t1175;
    let t39591 = t34884 * t9206;
    let t39595 = t27006 * t2295;
    let t39600 = t1970 * t1971 * t511 * t1475 * t848;
    (t39584, t39589, t39591, t39595, t39600)
}
