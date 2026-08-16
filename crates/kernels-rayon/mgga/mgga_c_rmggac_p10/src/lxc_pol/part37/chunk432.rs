//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 432/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk432(t2338: f64, t356: f64, t2164: f64, t574: f64, t1656: f64, t640: f64, t2402: f64, t333: f64, t1664: f64, t668: f64, t1614: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8854 = t2338 * t356;
    let t8858 = t2164 * t574;
    let t8862 = t640 * t1656;
    let t8866 = t2402 * t333;
    let t8876 = t1664 * t668;
    let t8884 = t645 * t1614;
    (t8854, t8858, t8862, t8866, t8876, t8884)
}
