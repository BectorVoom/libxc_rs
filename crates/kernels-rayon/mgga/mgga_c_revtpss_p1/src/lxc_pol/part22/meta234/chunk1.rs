//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1472/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1472(t3915: f64, t5722: f64, t1444: f64, t1903: f64, t4076: f64, t1882: f64, t555: f64) -> (f64, f64, f64) {
    let t5723 = t3915 * t5722;
    let t5727 = t1903 * t1444;
    let t5728 = t4076 * t5727;
    let t5735 = t555 * t1882;
    (t5723, t5728, t5735)
}
