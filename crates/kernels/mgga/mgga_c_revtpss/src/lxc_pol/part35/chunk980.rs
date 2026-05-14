//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 980/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk980<F: Float>(t233: F, t30379: F, t1957: F, t225: F, t2061: F, t5977: F, t2723: F, t25416: F, t231: F, t7076: F, t1558: F, t7997: F, t6016: F, t6048: F, t25317: F, t1956: F, t2067: F, t213: F, t257: F, t26534: F, t26536: F, t26538: F, t26557: F, t26578: F, t27199: F, t28422: F, t28434: F, t28449: F, t29698: F, t30357: F, t7070: F, t7766: F, t8007: F, t8016: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t30380 = t233 * t30379;
    let t30381 = t1957 * t30380;
    let t30384 = t30379 * t225;
    let t30390 = t2061 * t5977;
    let t30391 = t30390 * t2723;
    let t30392 = t25416 * t30391;
    let t30395 = t30390 * t231;
    let t30396 = t7076 * t30395;
    let t30400 = t7997 * t1558 * t231;
    let t30401 = t7076 * t30400;
    let t30405 = t2061 * t6016 * t231;
    let t30406 = t7076 * t30405;
    let t30410 = t2061 * t6048;
    let t30411 = t25317 * t30410;
    let t30418 = 0.8673628188205199462e0 * t7070 * t30357 - 0.8673628188205199462e0 * t7766 * t8016 - 0.4336814094102599731e0 * t1956 * t30381 - t26534 + 0.65854491829355115987e0 * t213 * t30384 * t257 - 0.4336814094102599731e0 * t29698 * t2067 - t26536 - 0.8673628188205199462e0 * t7070 * t30392 + 0.4336814094102599731e0 * t7070 * t30396 + 0.8673628188205199462e0 * t7070 * t30401 + 0.4336814094102599731e0 * t7070 * t30406 - t26538 + 0.14456046980341999104e-1 * t28422 - 0.26020884564615598386e1 * t7070 * t30411 - t26557 + 0.17347256376410398924e1 * t27199 * t8007 - 0.19514881078765566038e-1 * t28434 - 0.10975748638225852664e-1 * t28449 + t26578;
    (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400, t30401, t30405, t30406, t30410, t30411, t30418)
}
