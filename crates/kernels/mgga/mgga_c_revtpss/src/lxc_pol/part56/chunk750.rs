//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 750/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk750<F: Float>(t1949: F, t2718: F, t14587: F, t25383: F, t25388: F, t25391: F, t25400: F, t25406: F, t25414: F, t25424: F, t25432: F, t27335: F, t27338: F, t27342: F, t27344: F, t27350: F, t27353: F, t27354: F, t7083: F, t7766: F, t7770: F) -> (F, F) {
    let t27357 = t2718 * t1949;
    let t27358 = t27357 * t14587;
    let t27361 = 0.25702851531048074406e-1 * t25388 - 0.9757440539382783019e-2 * t25400 - 0.4336814094102599731e0 * t7766 * t7083 - t25406 + 0.54878743191129263322e-2 * t27335 + 0.12851425765524037203e-1 * t25414 + 0.72280234901709995518e-2 * t27338 + t25424 - 0.14456046980341999104e-1 * t27342 + 0.25702851531048074406e-1 * t27344 + 0.8673628188205199462e0 * t25383 * t7770 - 0.72280234901709995518e-2 * t25432 - 0.8673628188205199462e0 * t25391 * t27350 + 0.4336814094102599731e0 * t27353 * t27354 - 0.8673628188205199462e0 * t27353 * t27358;
    (t27357, t27361)
}
