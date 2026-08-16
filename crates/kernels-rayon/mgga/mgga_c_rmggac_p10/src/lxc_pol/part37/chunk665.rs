//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 665/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk665(t2227: f64, t558: f64, t1587: f64, t698: f64, t2447: f64, t321: f64, t333: f64, t623: f64, t8619: f64, t511: f64, t6477: f64, t2144: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44244 = t2447 * t321;
    let t44293 = t2447 * t333;
    let t44788 = t623 * t8619;
    let t45468 = t6477 * t511;
    let t52781 = t892 * t2144;
    (t44232, t44239, t44244, t44293, t44788, t45468, t52781)
}
