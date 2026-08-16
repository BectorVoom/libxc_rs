//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2029/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2029(t18797: f64, t26497: f64, t110322: f64, t25375: f64, t103467: f64, t103471: f64, t103490: f64, t103494: f64, t1558: f64, t231: f64, t25317: f64, t25383: f64, t27199: f64, t28340: f64, t28378: f64, t30379: f64, t30396: f64, t30401: f64, t4533: f64, t7070: f64, t7071: f64, t7076: f64, t8006: f64, t886: f64, t95899: f64, t95902: f64, t95905: f64) -> f64 {
    let t110613 = t26497 * t18797;
    let t110615 = t25375 * t110322;
    let t110635 = -0.52041769129231196772e1_f64 * t7070 * t25317 * t8006 * t4533 + t95899 + t103467 + 0.96373646535613327359e-3_f64 * t103471 - 0.9757440539382783019e-2_f64 * t110613 - 0.14456046980341999104e-1_f64 * t110615 + 0.4336814094102599731e0_f64 * t25383 * t30396 + 0.14634331517634470219e-1_f64 * t103490 + 0.45699670022203476294e-2_f64 * t95902 - t103494 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t30379 * t886 - 0.73171657588172351096e-2_f64 * t95905 + 0.8673628188205199462e0_f64 * t27199 * t28378 + 0.8673628188205199462e0_f64 * t25383 * t30401 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t28340 * t1558 * t231;
    t110635
}
