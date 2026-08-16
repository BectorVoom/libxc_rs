//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1233/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1233(t1622: f64, t35251: f64, t1633: f64, t11420: f64, t116: f64, t34021: f64, t11391: f64, t677: f64, t11412: f64, t169: f64, t4043: f64, t8960: f64) -> (f64, f64, f64, f64, f64) {
    let t35252 = t1622 * t35251;
    let t35254 = t1633 * t35251;
    let t35257 = t116 * t34021 * t11420;
    let t35259 = t11391 * t677;
    let t35263 = t169 * t11412 * t4043 * t8960;
    (t35252, t35254, t35257, t35259, t35263)
}
