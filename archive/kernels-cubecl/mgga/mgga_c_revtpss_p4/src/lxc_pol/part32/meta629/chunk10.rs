//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2029/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2029<F: Float>(t18797: F, t26497: F, t110322: F, t25375: F, t103467: F, t103471: F, t103490: F, t103494: F, t1558: F, t231: F, t25317: F, t25383: F, t27199: F, t28340: F, t28378: F, t30379: F, t30396: F, t30401: F, t4533: F, t7070: F, t7071: F, t7076: F, t8006: F, t886: F, t95899: F, t95902: F, t95905: F) -> F {
    let t110613 = t26497 * t18797;
    let t110615 = t25375 * t110322;
    let t110635 = -F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t8006 * t4533 + t95899 + t103467 + F::cast_from(0.96373646535613327359e-3_f64) * t103471 - F::cast_from(0.9757440539382783019e-2_f64) * t110613 - F::cast_from(0.14456046980341999104e-1_f64) * t110615 + F::cast_from(0.4336814094102599731e0_f64) * t25383 * t30396 + F::cast_from(0.14634331517634470219e-1_f64) * t103490 + F::cast_from(0.45699670022203476294e-2_f64) * t95902 - t103494 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t30379 * t886 - F::cast_from(0.73171657588172351096e-2_f64) * t95905 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t28378 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t30401 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t28340 * t1558 * t231;
    t110635
}
