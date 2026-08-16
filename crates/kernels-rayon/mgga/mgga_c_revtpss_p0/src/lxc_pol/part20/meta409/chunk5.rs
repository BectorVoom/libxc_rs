//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1518/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518(t41864: f64, t41867: f64, t41871: f64, t41873: f64, t41876: f64, t41879: f64, t41882: f64, t41885: f64, t41888: f64, t41942: f64, t41947: f64, t41949: f64) -> f64 {
    let t42850 = -t41942 + t41947 + t41949 - t41864 - t41867 + t41871 + t41873 - t41876 - t41879 - t41882 - t41885 + t41888;
    t42850
}
