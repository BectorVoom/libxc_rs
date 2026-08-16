//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1982/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1982(t102434: f64, t102439: f64, t102453: f64, t102458: f64, t102462: f64, t102465: f64, t108225: f64, t108282: f64, t108448: f64, t22433: f64, t25930: f64, t26304: f64, t27868: f64, t28911: f64, t28912: f64, t75012: f64, t7511: f64, t75267: f64, t7528: f64, t96456: f64, t96460: f64) -> f64 {
    let t109704 = -0.17347256376410398924e1_f64 * t108225 * t28912 + 0.26019841438354088051e-1_f64 * t102434 - 0.23131639038696784278e-2_f64 * t102439 + 0.45699670022203476294e-2_f64 * t96456 - t102453 - 0.39512695097613069591e1_f64 * t7511 * t22433 - t102458 + 0.14634331517634470219e-1_f64 * t102462 - t102465 + 0.13009920719177044025e-1_f64 * t96460 - 0.8673628188205199462e0_f64 * t27868 * t28911 * t75267 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t108448 + 0.8673628188205199462e0_f64 * t27868 * t26304 * t75012 + 0.4336814094102599731e0_f64 * t108282 * t7528;
    t109704
}
