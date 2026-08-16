//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 787/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk787(t5542: f64, t8601: f64, t674: f64, t8607: f64, t7715: f64, t35589: f64, t570: f64, t739: f64, t1609: f64, t1986: f64, t7244: f64, t8447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38370 = t8601 * t7715 * t674;
    let t38374 = t8607 * t7715 * t674;
    let t38381 = t35589 * t570;
    let t38382 = t739 * t38381;
    let t38397 = t1986 * t1609;
    let t38414 = t7244 * t8447;
    (t38350, t38351, t38354, t38355, t38370, t38374, t38381, t38382, t38397, t38414)
}
