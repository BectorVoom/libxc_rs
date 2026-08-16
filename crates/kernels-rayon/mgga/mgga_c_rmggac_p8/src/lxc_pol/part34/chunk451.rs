//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 451/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk451(t1624: f64, t236: f64, t1627: f64, t511: f64, t515: f64, t8377: f64, t495: f64, t558: f64, t109: f64, t4179: f64, t490: f64, t498: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9189 = t236 * t1624;
    let t9193 = t511 * t1627;
    let t9197 = t515 * t8377;
    let t9205 = t511 * t558 * t495;
    let t9209 = t4179 * t109;
    let t9210 = t490 * t9209;
    let t9211 = t618 * t498;
    (t9189, t9193, t9197, t9205, t9209, t9210, t9211)
}
