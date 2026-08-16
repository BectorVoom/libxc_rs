//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1975/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1975(t102409: f64, t102411: f64, t102422: f64, t102434: f64, t102439: f64, t13920: f64, t2097: f64, t25930: f64, t26304: f64, t27868: f64, t28855: f64, t49376: f64, t543: f64, t7295: f64, t7301: f64, t7523: f64, t96432: f64, t96437: f64, t97742: f64, t97839: f64, t97855: f64, t98050: f64, t98299: f64) -> f64 {
    let t102443 = -0.72280234901709995518e-2_f64 * t96432 - 0.96373646535613327357e-2_f64 * t102409 + 0.17135234354032049604e-1_f64 * t102411 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t97742 - 0.17347256376410398924e1_f64 * t25930 * t26304 * t97839 - t102422 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2097 * t13920 * t543 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t98299 - 0.10975748638225852664e-1_f64 * t96437 + 0.17347256376410398924e1_f64 * t98050 * t7523 + 0.13009920719177044025e-1_f64 * t102434 + 0.8673628188205199462e0_f64 * t27868 * t26304 * t49376 - 0.11565819519348392139e-2_f64 * t102439 + 0.8673628188205199462e0_f64 * t97855 * t28855;
    t102443
}
