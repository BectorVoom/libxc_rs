//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 525/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk525<F: Float>(t2327: F, t94: F, t1310: F, t670: F, t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t665: F, t613: F, tau0: F) -> (F, F, F, F, F, F, F, F) {
    let t2328 = t94 * t2327;
    let t2331 = t1310 * t670;
    let t2335 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = F::cast_from(1.0_f64) / t654 / t111;
    let t2340 = t665 * t665;
    let t2341 = t2339 * t2340;
    let t2344 = tau0 * t613;
    (t2328, t2331, t2335, t2336, t2339, t2340, t2341, t2344)
}
