//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 783/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk783(t4759: f64, t732: f64, t2367: f64, t5075: f64, t999: f64, t3974: f64, t4054: f64, t5064: f64, t2472: f64, t4919: f64, t4037: f64, t4053: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13578 = t732 * t4759;
    let t13602 = t2367 * t5075;
    let t13603 = t999 * t13602;
    let t13607 = t4054 * t3974;
    let t13611 = t2367 * t5064;
    let t13612 = t999 * t13611;
    let t13614 = t2472 * t4919;
    let t13632 = t4053 * t4037;
    (t13578, t13602, t13603, t13607, t13611, t13612, t13614, t13632)
}
