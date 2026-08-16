//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 662/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk662(t40927: f64, t797: f64, t838: f64, t1614: f64, t664: f64, t1587: f64, t2067: f64, t26: f64, t2367: f64, t333: f64, t1652: f64, t2123: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40928 = t797 * t40927;
    let t40932 = t838 * t40927;
    let t40940 = t664 * t1614;
    let t40983 = t664 * t1587;
    let t40998 = t2067 * t26;
    let t41006 = t2367 * t333;
    let t41015 = t664 * t1652;
    let t41059 = t2123 * t551;
    (t40928, t40932, t40940, t40983, t40998, t41006, t41015, t41059)
}
