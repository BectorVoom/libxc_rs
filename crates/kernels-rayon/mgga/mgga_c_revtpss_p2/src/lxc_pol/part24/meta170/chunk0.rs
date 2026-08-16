//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 837/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk837(t247: f64, t6678: f64, t1264: f64, t6425: f64, t1774: f64, t1794: f64, t1250: f64, t3720: f64) -> (f64, f64, f64, f64, f64) {
    let t6679 = t247 * t6678;
    let t6682 = t1264 * t6425;
    let t6683 = t247 * t6682;
    let t6688 = t1774 * t1794;
    let t6689 = t6688 * t1250;
    let t6690 = t3720 * t6689;
    (t6679, t6683, t6688, t6689, t6690)
}
