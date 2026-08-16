//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 756/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk756(t1633: f64, t8822: f64, t1860: f64, t3103: f64, t3105: f64, t1030: f64, t3717: f64, t1749: f64, t1736: f64, t3131: f64, t1743: f64, t3060: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9011 = t1633 * t8822;
    let t9013 = t1860 * t3103;
    let t9014 = t9013 * t3105;
    let t9016 = t1030 * t3717;
    let t9017 = t9016 * t1749;
    let t9019 = t3131 * t1736;
    let t9020 = t1743 * t9019;
    let t9021 = t9020 * t1749;
    let t9023 = t3060 * t3127;
    (t9011, t9014, t9016, t9017, t9019, t9020, t9021, t9023)
}
