//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1657/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1657(t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64) -> (f64, f64) {
    let t45231 = -0.25367901234567901233e-1_f64 * t43858 - 0.50735802469135802467e-1_f64 * t43862 - 0.13698666666666666667e0_f64 * t43830 - 0.3044148148148148148e-1_f64 * t43865 + 0.4566222222222222222e-1_f64 * t43832 + 0.11415555555555555555e0_f64 * t43837 - 0.34246666666666666665e-1_f64 * t43871 - 0.4566222222222222222e-1_f64 * t43841 + 0.61644e0_f64 * t43845 + 0.10274e0_f64 * t43877 + 0.13698666666666666667e0_f64 * t43849;
    let t45232 = 0.17757530864197530864e0_f64 * t43813;
    (t45231, t45232)
}
