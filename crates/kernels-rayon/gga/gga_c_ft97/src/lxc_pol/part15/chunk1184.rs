//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1184/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1184(t292: f64, t1209: f64, t14721: f64, t14766: f64, t22085: f64, t2691: f64, t2725: f64, t285: f64, t291: f64, t4113: f64, t43586: f64, t5003: f64, t5232: f64, t5284: f64, t70487: f64, t70671: f64, t800: f64, t82848: f64, t89941: f64, t89994: f64, t89999: f64, t90003: f64, t90008: f64, t90015: f64, t90054: f64, t90088: f64, t90168: f64, t90204: f64, t90234: f64, t90264: f64, t90300: f64) -> f64 {
    let t293 = 0.1e-59_f64 < t292;
    let t90304 = piecewise3(t293, 2.0_f64 * t800 * t291 * (t89941 + t89994) + 24.0_f64 * t285 * t43586 * t89999 + 6.0_f64 * t285 * t2725 * t90003 - 0.14498192132169191472e2_f64 * t14766 * t90008 - 0.45910941751869106328e2_f64 * t5232 * t5003 + 0.14498192132169191472e2_f64 * t14721 * t90008 - 0.4127938044770952877e1_f64 * t4113 * t90015 + 24.0_f64 * t2691 * t70487 * t5284 - 24.0_f64 * t70671 * t22085 - 0.65177969127962413846e0_f64 * t82848 * t1209 + t90054 + t90088 + t90168 + t90204 + t90234 + t90264 + t90300, 0.0_f64);
    t90304
}
