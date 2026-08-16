//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 979/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk979(t7990: f64, t8061: f64, t3915: f64, t8347: f64, t2217: f64, t862: f64, t865: f64, t2131: f64, t2147: f64, t463: f64, t8103: f64, t2176: f64, t3889: f64) -> (f64, f64, f64, f64, f64) {
    let t33034 = t7990 * t8061;
    let t33037 = 0.39512695097613069591e1_f64 * t8347 * t3915;
    let t33047 = t862 * t2217 * t865;
    let t33053 = t2131 * t2147 * t8103 * t463;
    let t33063 = t2176 * t3889;
    (t33034, t33037, t33047, t33053, t33063)
}
