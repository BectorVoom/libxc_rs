//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1092/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1092<F: Float>(t30400: F, t7076: F, t2061: F, t231: F, t6016: F, t6048: F, t25317: F, t1956: F, t2067: F, t213: F, t257: F, t26534: F, t26536: F, t26538: F, t26557: F, t26578: F, t27199: F, t28422: F, t28434: F, t28449: F, t29698: F, t30357: F, t30381: F, t30384: F, t30392: F, t30396: F, t7070: F, t7766: F, t8007: F, t8016: F) -> (F, F, F, F, F, F) {
    let t30401 = t7076 * t30400;
    let t30405 = t2061 * t6016 * t231;
    let t30406 = t7076 * t30405;
    let t30410 = t2061 * t6048;
    let t30411 = t25317 * t30410;
    let t30418 = F::cast_from(0.8673628188205199462e0_f64) * t7070 * t30357 - F::cast_from(0.8673628188205199462e0_f64) * t7766 * t8016 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t30381 - t26534 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t30384 * t257 - F::cast_from(0.4336814094102599731e0_f64) * t29698 * t2067 - t26536 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t30392 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t30396 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t30401 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t30406 - t26538 + F::cast_from(0.14456046980341999104e-1_f64) * t28422 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t30411 - t26557 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t8007 - F::cast_from(0.19514881078765566038e-1_f64) * t28434 - F::cast_from(0.10975748638225852664e-1_f64) * t28449 + t26578;
    (t30401, t30405, t30406, t30410, t30411, t30418)
}
