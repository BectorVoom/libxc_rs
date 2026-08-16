//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 685/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk685(t13253: f64, t2343: f64, t11167: f64, t2325: f64, t883: f64, t882: f64, t11254: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t13254 = t2343 * t13253;
    let t13258 = t2325 * t883 * t11167;
    let t13259 = t882 * t13258;
    let t13260 = 0.11856252764865062333e-2_f64 * t13259;
    let t13261 = t11254 * t874;
    (t13254, t13258, t13260, t13261)
}
