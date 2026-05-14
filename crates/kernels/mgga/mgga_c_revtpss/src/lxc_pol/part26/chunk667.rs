//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 667/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk667<F: Float>(t7284: F, t7515: F, t7289: F, t1444: F, t2097: F, t7296: F, t1398: F, t543: F, t7301: F, t545: F, t7506: F, t2028: F, t1445: F, t2027: F, t2103: F, t213: F, t561: F, t7292: F, t7295: F, t7495: F, t7498: F, t7507: F, t7511: F) -> (F, F, F, F, F, F, F) {
    let t7517 = 0.72280234901709995518e-2 * t7284 * t7515;
    let t7519 = 0.12851425765524037203e-1 * t7289 * t7515;
    let t7522 = t2097 * t1444;
    let t7523 = t7296 * t7522;
    let t7527 = t2097 * t1398 * t543;
    let t7528 = t7301 * t7527;
    let t7531 = t545 * t7506;
    let t7532 = t2028 * t7531;
    let t7535 = -t7495 + t7498 + 0.65854491829355115987e0 * t213 * t7507 * t561 - 0.65854491829355115987e0 * t7511 * t1445 + t7517 - t7519 - 0.4336814094102599731e0 * t7292 * t2103 + 0.8673628188205199462e0 * t7295 * t7523 + 0.4336814094102599731e0 * t7295 * t7528 - 0.4336814094102599731e0 * t2027 * t7532;
    (t7522, t7523, t7527, t7528, t7531, t7532, t7535)
}
