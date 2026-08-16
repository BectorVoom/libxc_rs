//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2215/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2215(t22986: f64, t23270: f64, t865: f64, t98253: f64, t1528: f64, t2597: f64, t28311: f64, t866: f64, t86951: f64, t86968: f64, t86988: f64, t92432: f64, t98234: f64, t98237: f64, t98239: f64, t98248: f64, t98251: f64) -> f64 {
    let t98256 = t22986 * t23270 * t98253 * t865;
    let t98258 = 0.49348022005446793095e-1_f64 * t98234 - 0.24674011002723396548e-1_f64 * t98237 - 2.0_f64 * t98239 * t866 - 6.0_f64 * t2597 * t28311 + t86951 - 2.0_f64 * t86988 * t1528 + t92432 + 0.3289868133696452873e-1_f64 * t98248 - t86968 + 0.3289868133696452873e-1_f64 * t98251 + 0.16449340668482264365e-1_f64 * t98256;
    t98258
}
