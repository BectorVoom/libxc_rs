//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1066/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1066(t151230: f64, t151247: f64, t151264: f64, t151278: f64, t151296: f64, t151312: f64, t151327: f64, t151344: f64, t1403: f64, t141524: f64, t141527: f64, t141543: f64, t1427: f64, t1454: f64, t151139: f64, t151188: f64, t151200: f64, t151212: f64, t193: f64, t2: f64, t258: f64, t26: f64, t27894: f64, t27906: f64, t27908: f64, t33568: f64, t35276: f64, t4: f64, t5996: f64, t6840: f64, t7437: f64, t7487: f64) -> (f64, f64) {
    let t151347 = t151230 + t151247 + t151264 + t151278 + t151296 + t151312 + t151327 + t151344;
    let t151350 = (t151139 + t151188) * t2 * t4 * t26 * t1427 / 6.0_f64 + t7437 * t27908 / 6.0_f64 - t141524 / 18.0_f64 - t141527 / 18.0_f64 + t151200 / 9.0_f64 + t27894 * t7487 / 6.0_f64 + t5996 * t35276 / 3.0_f64 + t1403 * t193 * t27906 * t1454 / 3.0_f64 + t33568 * t6840 / 6.0_f64 + t151212 / 9.0_f64 + 2.0_f64 * t151347 * t258 + t141543;
    (t151347, t151350)
}
