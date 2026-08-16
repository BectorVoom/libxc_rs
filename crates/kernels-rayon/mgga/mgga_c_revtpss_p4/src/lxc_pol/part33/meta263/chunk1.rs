//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1178/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1178(t1398: f64, t2022: f64, t543: f64, t7301: f64, t545: f64, t7274: f64, t2028: f64, t1445: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7275: f64, t7279: f64, t7288: f64, t7291: f64, t7292: f64, t7295: f64, t7298: f64) -> (f64, f64, f64, f64, f64) {
    let t7303 = t2022 * t1398 * t543;
    let t7304 = t7301 * t7303;
    let t7307 = t545 * t7274;
    let t7308 = t2028 * t7307;
    let t7311 = -t7245 + t7248 + 0.65854491829355115987e0_f64 * t213 * t7275 * t561 - 0.65854491829355115987e0_f64 * t7279 * t1445 + t7288 - t7291 - 0.4336814094102599731e0_f64 * t7292 * t2030 + 0.8673628188205199462e0_f64 * t7295 * t7298 + 0.4336814094102599731e0_f64 * t7295 * t7304 - 0.4336814094102599731e0_f64 * t2027 * t7308;
    (t7303, t7304, t7307, t7308, t7311)
}
