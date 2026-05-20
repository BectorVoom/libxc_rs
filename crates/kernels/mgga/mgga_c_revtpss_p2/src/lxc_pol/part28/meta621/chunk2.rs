//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2191/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2191<F: Float>(t25331: F, t27213: F, t93190: F, t99211: F, t25374: F, t98848: F, t25378: F, t99403: F, t231: F, t2645: F, t27265: F, t7070: F, t7076: F, t7759: F, t836: F, t93326: F, t93331: F, t93334: F, t93335: F, t93337: F, t93339: F, t93343: F, t93346: F, t93365: F) -> F {
    let t99456 = t27213 * t25331;
    let t99460 = t93190 * t99211;
    let t99463 = t98848 * t25374;
    let t99465 = F::cast_from(0.51405703062096148812e-1_f64) * t99463 * t25378;
    let t99466 = t99403 * t25374;
    let t99468 = F::cast_from(0.28912093960683998208e-1_f64) * t99466 * t25378;
    let t99469 = F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t27265 * t836 * t231 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t7759 * t2645 * t231 - F::cast_from(0.14456046980341999104e-1_f64) * t93326 - F::cast_from(0.28912093960683998208e-1_f64) * t93331 - t93334 - F::cast_from(0.34270468708064099208e-1_f64) * t93335 - F::cast_from(0.72280234901709995518e-2_f64) * t93337 - F::cast_from(0.68540937416128198416e-1_f64) * t93339 - F::cast_from(0.96373646535613327357e-2_f64) * t99456 + F::cast_from(0.51405703062096148812e-1_f64) * t93343 - F::cast_from(0.9757440539382783019e-2_f64) * t93346 + F::cast_from(0.45699670022203476294e-2_f64) * t99460 - F::cast_from(0.28912093960683998208e-1_f64) * t93365 + t99465 - t99468;
    t99469
}
