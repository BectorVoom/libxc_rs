//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 525/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk525(t2176: f64, t2177: f64, t2175: f64, t128: f64, t131: f64, t136: f64, t124: f64, t137: f64, t68: f64, t209: f64, t660: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2178 = t2176 * t2177;
    let t2179 = t2175 * t2178;
    let t2184 = 1.0_f64 / t131 / t128 * t136;
    let t2185 = t137 * t124;
    let t2186 = t2185 * t68;
    let t2187 = t2184 * t2186;
    let t2189 = t660 * t209;
    let t2190 = t659 * t2189;
    (t2179, t2184, t2185, t2186, t2187, t2189, t2190)
}
