//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1200/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1200(t1165: f64, t12991: f64, t4267: f64, t5099: f64, t13087: f64, t6090: f64, t4389: f64, t5743: f64, t1180: f64, t13274: f64, t13276: f64, t13278: f64, t13280: f64, t13282: f64, t16765: f64, t16769: f64, t16779: f64, t16781: f64, t1879: f64, t955: f64) -> f64 {
    let t21832 = t12991 * t1165 * t4267 * t5099;
    let t21834 = t13087 * t6090;
    let t21844 = t4389 * t5743;
    let t21847 = -0.12862205435420921092e-2_f64 * t1180 * t1165 * t1879 * t955 + 0.68598428988911579156e-2_f64 * t21832 + 0.32012600194825403606e-1_f64 * t21834 - 0.13719685797782315831e-1_f64 * t16765 - 0.68598428988911579156e-2_f64 * t16769 + 455.0_f64 / 648.0_f64 * t13274 - 35.0_f64 / 108.0_f64 * t13276 - 35.0_f64 / 216.0_f64 * t13278 - 35.0_f64 / 216.0_f64 * t13280 - 35.0_f64 / 432.0_f64 * t13282 - 0.17149607247227894789e-2_f64 * t16779 + 0.16006300097412701803e-1_f64 * t21844 - 0.34299214494455789577e-2_f64 * t16781;
    t21847
}
