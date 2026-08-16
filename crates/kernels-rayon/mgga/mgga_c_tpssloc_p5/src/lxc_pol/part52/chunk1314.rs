//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1314/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1314(t33133: f64, t7000: f64, t33160: f64, t6876: f64, t26502: f64, t3701: f64, t1983: f64, t2019: f64, t26142: f64, t6517: f64, t25994: f64, t19456: f64, t8323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120005 = t33133 * t7000;
    let t120008 = 3.0_f64 * t6876 * t33160;
    let t120016 = t3701 * t26502;
    let t120019 = 2.0_f64 * t1983 * t2019 * t120016;
    let t120020 = t6517 * t26142;
    let t120022 = t6517 * t25994;
    let t120027 = t19456 * t8323;
    (t120005, t120008, t120019, t120020, t120022, t120027)
}
