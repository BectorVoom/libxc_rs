//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 221/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk221(t1423: f64, t1538: f64, t109: f64, t321: f64, t571: f64, t333: f64, t117: f64, t899: f64) -> (f64, f64, f64, f64, f64) {
    let t1539 = t1423 + t1538;
    let t1540 = t1539 * t109;
    let t1544 = t571 * t321;
    let t1547 = t571 * t333;
    let t1550 = t899 * t117;
    (t1539, t1540, t1544, t1547, t1550)
}
