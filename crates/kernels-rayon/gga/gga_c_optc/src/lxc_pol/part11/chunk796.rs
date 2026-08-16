//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk796(t2641: f64, t4961: f64, t2669: f64, t24: f64, t4933: f64, t862: f64, t4937: f64, t4929: f64, t4983: f64, t907: f64, t106: f64, t1392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14360 = t2641 * t4961;
    let t14390 = t2669 * t4961;
    let t14420 = t24 * t4933;
    let t14421 = t862 * t14420;
    let t14425 = t24 * t4937;
    let t14426 = t862 * t14425;
    let t14430 = t24 * t4929;
    let t14431 = t862 * t14430;
    let t14472 = t4983 * t907;
    let t14479 = t106 * t1392;
    (t14360, t14390, t14421, t14426, t14431, t14472, t14479)
}
