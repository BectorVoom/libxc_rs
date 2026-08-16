//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1848/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1848(t1976: f64, t5015: f64, t7160: f64, t3046: f64, t7143: f64, t1032: f64, t1678: f64) -> (f64, f64, f64, f64) {
    let t27411 = t1976 * t5015;
    let t27412 = t7160 * t27411;
    let t27415 = t3046 * t7143;
    let t27418 = t1678 * t1032;
    (t27411, t27412, t27415, t27418)
}
