//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1225/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1225(t100974: f64, t100981: f64, t100987: f64, t121716: f64, t121751: f64, t127218: f64, t127593: f64, t127914: f64, t127966: f64, t1711: f64, t1940: f64, t25759: f64, t26425: f64, t27770: f64, t27793: f64, t27799: f64, t27806: f64, t28291: f64, t28472: f64, t32487: f64, t32491: f64, t32498: f64, t34097: f64, t7869: f64) -> f64 {
    let t128097 = t1940 * t32487 * t1711 / 2.0_f64 + t28472 * t127218 + t28472 * t100974 * t34097 - 3.0_f64 * t28472 * t100981 * t127914 - 3.0_f64 * t28291 * t25759 * t127966 - 3.0_f64 / 2.0_f64 * t26425 * t100987 * t32498 - t1940 * t32491 * t27806 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t121751 * t27793 - t1940 * t121716 * t7869 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t121751 * t27770 + t28472 * t27799 * t127593;
    t128097
}
