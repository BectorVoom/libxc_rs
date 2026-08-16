//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 579/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk579(t2453: f64, t267: f64, t2488: f64, t2496: f64, t854: f64, t235: f64, t68: f64, t275: f64, t277: f64, t673: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2499 = 0.39862222222222222223e0_f64 * t2453;
    let t2504 = 1.0_f64/f64::sqrt(t267);
    let t2505 = t2504 * t2488;
    let t2507 = t854 * t2496;
    let t2509 = t68 * t235;
    let t2511 = t275 * t2509 * t277;
    let t2512 = 0.13692777777777777778e0_f64 * t2511;
    let t2513 = t673 * t862;
    (t2499, t2504, t2505, t2507, t2509, t2511, t2512, t2513)
}
