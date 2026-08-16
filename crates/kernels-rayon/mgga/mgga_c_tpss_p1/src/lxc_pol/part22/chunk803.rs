//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 803/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk803(t1219: f64, t1649: f64, t1233: f64, t1260: f64, t1640: f64, t220: f64, t3374: f64, t339: f64, t4417: f64, t4460: f64, t4487: f64, t4498: f64, t4499: f64, t4508: f64, t523: f64) -> (f64, f64) {
    let t4511 = t1219 * t1649;
    let t4516 = -t1233 * t339 * t4511 - t1233 * t4499 * t4508 - t1260 * t339 * t4460 - t1640 * t3374 * t339 + t220 * t4487 * t523 + 2.0_f64 * t4417 * t4498 * t4499;
    (t4511, t4516)
}
