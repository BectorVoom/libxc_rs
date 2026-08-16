//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1147/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1147(t1439: f64, t1983: f64, t7380: f64, t1460: f64, t1992: f64, t2095: f64, t30225: f64, t532: f64, t1569: f64, t7605: f64, t2001: f64, t5237: f64) -> (f64, f64, f64, f64, f64) {
    let t36364 = t7380 * t1983 * t1439;
    let t36367 = t2095 * t1992 * t1460;
    let t36370 = t30225 * t532;
    let t36372 = t7605 * t1569;
    let t36374 = t2001 * t5237;
    (t36364, t36367, t36370, t36372, t36374)
}
