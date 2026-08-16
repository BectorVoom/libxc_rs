//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1949/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1949(t25994: f64, t652: f64, t2314: f64, t7468: f64, t25965: f64, t25969: f64, t25973: f64, t25975: f64, t25977: f64, t25979: f64, t25982: f64, t25987: f64, t25991: f64, t25993: f64, t4028: f64, t4034: f64, t650: f64, t6539: f64, t7472: f64, t7670: f64) -> f64 {
    let t25996 = 2.0_f64 * t652 * t25994;
    let t25998 = 2.0_f64 * t2314 * t7468;
    let t25999 = -2.0_f64 * t25965 * t652 - 2.0_f64 * t4028 * t6539 - 2.0_f64 * t4034 * t7472 - t650 * t7670 - t25969 - t25973 - t25975 - t25977 - t25979 - t25982 + t25987 - t25991 - t25993 - t25996 - t25998;
    t25999
}
