//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1628/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1628(t18188: f64, t19288: f64, t12560: f64, t12561: f64, t12562: f64, t12563: f64, t12564: f64, t12565: f64, t9225: f64, t5385: f64, t604: f64, t5389: f64, t645: f64) -> (f64, f64, f64, f64) {
    let t19289 = t18188 + t19288;
    let t19297 = t12560 + t12561 + t12562 + t12563 + t12564 - t12565 - t9225;
    let t19299 = t5385 * t604;
    let t19310 = t5389 * t645;
    (t19289, t19297, t19299, t19310)
}
