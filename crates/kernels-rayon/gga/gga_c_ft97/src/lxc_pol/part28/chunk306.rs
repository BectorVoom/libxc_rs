//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 306/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk306(t1013: f64, t2057: f64, t554: f64, t1722: f64, t1733: f64, t2066: f64, t3083: f64, t3086: f64, t3090: f64, t3093: f64, t3097: f64) -> (f64, f64) {
    let t3393 = t2057 * t1013;
    let t3394 = t3393 * t554;
    let t3404 = -0.44452000728395061731e-1_f64 * t1722 - t2066 + 0.55565000910493827163e-2_f64 * t1733 - 0.44452000728395061731e-1_f64 * t3083 + 0.55565000910493827163e-2_f64 * t3086 + 0.22226000364197530865e-1_f64 * t3090 - 0.33339000546296296298e-1_f64 * t3093 + 0.33339000546296296298e-1_f64 * t3097;
    (t3394, t3404)
}
