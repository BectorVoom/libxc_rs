//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2046/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2046(t13730: f64, t2023: f64, t2782: f64, t10073: f64, t25938: f64, t27836: f64, t14079: f64, t26054: f64, t7289: f64, t97925: f64, t2470: f64, t27872: f64) -> (f64, f64, f64, f64, f64) {
    let t98001 = 0.21951497276451705328e-1_f64 * t2782 * t2023 * t13730;
    let t98003 = t10073 * t27836 * t25938;
    let t98010 = 0.19514881078765566038e-1_f64 * t26054 * t14079;
    let t98011 = t7289 * t97925;
    let t98028 = t27872 * t2470;
    (t98001, t98003, t98010, t98011, t98028)
}
