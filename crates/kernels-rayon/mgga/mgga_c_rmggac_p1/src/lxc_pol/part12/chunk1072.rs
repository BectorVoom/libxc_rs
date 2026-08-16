//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1072/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1072(t1175: f64, t236: f64, t618: f64, t7231: f64, t8517: f64, t34884: f64, t9123: f64, t1240: f64, t1971: f64, t511: f64, t558: f64, t7230: f64) -> (f64, f64, f64) {
    let t42142 = t8517 * t7231 * t236 * t618 * t1175;
    let t42144 = t34884 * t9123;
    let t42145 = 0.24829349937757072982e-4_f64 * t42144;
    let t42149 = t7230 * t1971 * t511 * t558 * t1240;
    (t42142, t42145, t42149)
}
