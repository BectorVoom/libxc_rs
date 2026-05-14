//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 971/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk971<F: Float>(t26567: F, t7076: F, t2061: F, t231: F, t2645: F, t2062: F, t2453: F, t2458: F, t2067: F, t25383: F, t25391: F, t25407: F, t26529: F, t26534: F, t26536: F, t26538: F, t26541: F, t26545: F, t26547: F, t26551: F, t26557: F, t26558: F, t26561: F, t26564: F, t2772: F, t2829: F, t7070: F, t7403: F, t7420: F, t887: F) -> (F, F, F, F) {
    let t26568 = t7076 * t26567;
    let t26573 = t7076 * t2061 * t2645 * t231;
    let t26576 = t2453 * t2062;
    let t26578 = 0.11565819519348392139e-2 * t26576 * t2458;
    let t26579 = 0.8673628188205199462e0 * t25383 * t7420 - 0.65854491829355115987e0 * t7403 * t2829 + 0.14456046980341999104e-1 * t26529 + 0.13170898365871023197e1 * t7403 * t2772 - t26534 - t26536 - t26538 - 0.4336814094102599731e0 * t25407 * t2067 - 0.28912093960683998208e-1 * t26541 + 0.14456046980341999104e-1 * t26545 - 0.13170898365871023197e1 * t26547 * t887 - 0.17347256376410398924e1 * t25391 * t26551 - t26557 - 0.25702851531048074406e-1 * t26558 + 0.10975748638225852664e-1 * t26561 + 0.19514881078765566038e-1 * t26564 + 0.8673628188205199462e0 * t7070 * t26568 + 0.4336814094102599731e0 * t7070 * t26573 + t26578;
    (t26568, t26573, t26576, t26579)
}
