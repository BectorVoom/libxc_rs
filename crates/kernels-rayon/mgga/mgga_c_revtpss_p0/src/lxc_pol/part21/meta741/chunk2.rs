//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2609/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2609(t13790: f64, t4056: f64, t10022: f64, t2782: f64, t10073: f64, t14231: f64, t10139: f64, t14219: f64, t9285: f64, t14215: f64, t2470: f64, t4101: f64) -> (f64, f64, f64, f64) {
    let t48025 = t13790 * t4056;
    let t48027 = t2782 * t10022 * t48025;
    let t48029 = t10073 * t14231;
    let t48036 = t10139 * t14219 * t9285;
    let t48039 = t4101 * t14215 * t2470;
    (t48027, t48029, t48036, t48039)
}
