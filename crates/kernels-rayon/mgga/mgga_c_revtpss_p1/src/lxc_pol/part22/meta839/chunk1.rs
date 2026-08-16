//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2969/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969(t13850: f64, t2482: f64, t2668: f64, t4000: f64, t13841: f64, t9962: f64, t4010: f64, t808: f64, t13785: f64, t48862: f64, t13817: f64, t13999: f64) -> (f64, f64, f64, f64, f64) {
    let t48982 = t2482 * t4000 * t2668 * t13850;
    let t48984 = t9962 * t13841;
    let t48999 = t808 * t4010;
    let t49001 = t48862 * t48999 * t13785;
    let t49003 = t13999 * t13817;
    (t48982, t48984, t48999, t49001, t49003)
}
