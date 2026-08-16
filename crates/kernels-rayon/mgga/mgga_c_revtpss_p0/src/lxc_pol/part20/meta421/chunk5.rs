//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1577/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577(t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64) -> (f64, f64) {
    let t43880 = -40.0_f64 / 81.0_f64 * t43858 - 80.0_f64 / 81.0_f64 * t43862 - 8.0_f64 / 3.0_f64 * t43830 - 16.0_f64 / 27.0_f64 * t43865 + 8.0_f64 / 9.0_f64 * t43832 + 20.0_f64 / 9.0_f64 * t43837 - 2.0_f64 / 3.0_f64 * t43871 - 8.0_f64 / 9.0_f64 * t43841 + 12.0_f64 * t43845 + 2.0_f64 * t43877 + 8.0_f64 / 3.0_f64 * t43849;
    let t43881 = 280.0_f64 / 81.0_f64 * t43813;
    (t43880, t43881)
}
