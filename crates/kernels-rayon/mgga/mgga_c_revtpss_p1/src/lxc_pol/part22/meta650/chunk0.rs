//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2595/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2595(t3801: f64, t6748: f64, t1209: f64, t6695: f64, t460: f64, t1214: f64, t6587: f64) -> (f64, f64, f64, f64) {
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20703 = t6587 * t1214;
    (t20692, t20697, t20700, t20703)
}
