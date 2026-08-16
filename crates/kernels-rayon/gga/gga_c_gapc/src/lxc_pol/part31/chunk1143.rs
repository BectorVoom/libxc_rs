//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1143/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1143(t28924: f64, t3784: f64, t11937: f64, t2639: f64, t11889: f64, t16408: f64, t612: f64, t11887: f64, t7956: f64, t818: f64, t9066: f64, t11986: f64, t7939: f64) -> (f64, f64, f64, f64, f64) {
    let t33242 = t3784 * t28924;
    let t33245 = t11937 * t2639;
    let t33248 = t16408 * t612 * t11889;
    let t33252 = t11887 * t9066 * t818 * t7956;
    let t33254 = t11986 * t7939;
    (t33242, t33245, t33248, t33252, t33254)
}
