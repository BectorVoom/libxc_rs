//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3377/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377(t15220: f64, t4598: f64, t18984: f64, t2889: f64, t18987: f64, t4614: f64, t18992: f64, t18950: f64, t2880: f64, t918: f64, t2897: f64, t2881: f64, t41401: f64, t6113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63474 = t4598 * t15220;
    let t63476 = t18984 * t2889;
    let t63478 = t18987 * t2889;
    let t63480 = t4614 * t15220;
    let t63482 = t18992 * t2889;
    let t63485 = t2880 * t18950 * t918;
    let t63488 = t2897 * t18950 * t918;
    let t63491 = t41401 * t6113 * t2881;
    (t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491)
}
