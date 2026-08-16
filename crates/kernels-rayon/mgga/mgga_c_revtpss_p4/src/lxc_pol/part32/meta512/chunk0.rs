//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1805/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1805(t30400: f64, t7076: f64, t2061: f64, t231: f64, t6016: f64, t6048: f64, t25317: f64, t1956: f64, t2067: f64, t213: f64, t257: f64, t26534: f64, t26536: f64, t26538: f64, t26557: f64, t26578: f64, t27199: f64, t28422: f64, t28434: f64, t28449: f64, t29698: f64, t30357: f64, t30381: f64, t30384: f64, t30392: f64, t30396: f64, t7070: f64, t7766: f64, t8007: f64, t8016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30401 = t7076 * t30400;
    let t30405 = t2061 * t6016 * t231;
    let t30406 = t7076 * t30405;
    let t30410 = t2061 * t6048;
    let t30411 = t25317 * t30410;
    let t30418 = 0.8673628188205199462e0_f64 * t7070 * t30357 - 0.8673628188205199462e0_f64 * t7766 * t8016 - 0.4336814094102599731e0_f64 * t1956 * t30381 - t26534 + 0.65854491829355115987e0_f64 * t213 * t30384 * t257 - 0.4336814094102599731e0_f64 * t29698 * t2067 - t26536 - 0.8673628188205199462e0_f64 * t7070 * t30392 + 0.4336814094102599731e0_f64 * t7070 * t30396 + 0.8673628188205199462e0_f64 * t7070 * t30401 + 0.4336814094102599731e0_f64 * t7070 * t30406 - t26538 + 0.14456046980341999104e-1_f64 * t28422 - 0.26020884564615598386e1_f64 * t7070 * t30411 - t26557 + 0.17347256376410398924e1_f64 * t27199 * t8007 - 0.19514881078765566038e-1_f64 * t28434 - 0.10975748638225852664e-1_f64 * t28449 + t26578;
    (t30401, t30405, t30406, t30410, t30411, t30418)
}
