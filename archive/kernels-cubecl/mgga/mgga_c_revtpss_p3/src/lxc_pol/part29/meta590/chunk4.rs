//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1962/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962<F: Float>(t2470: F, t28779: F, t25895: F, t102185: F, t102205: F, t102213: F, t102217: F, t1398: F, t1444: F, t1903: F, t25924: F, t26079: F, t26333: F, t26343: F, t27837: F, t28862: F, t28888: F, t4003: F, t4056: F, t543: F, t7295: F, t7296: F, t7301: F, t8085: F, t96232: F, t96234: F, t96237: F, t96240: F) -> (F, F) {
    let t102218 = t28779 * t2470;
    let t102219 = t25895 * t102218;
    let t102222 = F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t28888 * t1398 * t543 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t28862 * t1444 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26079 * t102185 * t4003 - F::cast_from(0.8673628188205199462e0_f64) * t27837 * t26343 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t102185 * t543 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t8085 * t4056 * t543 + F::cast_from(0.72280234901709995518e-2_f64) * t96232 + F::cast_from(0.25702851531048074406e-1_f64) * t96234 + F::cast_from(0.45699670022203476294e-2_f64) * t102205 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t26333 * t1903 - F::cast_from(0.51405703062096148812e-1_f64) * t96237 + t102213 - t102217 + F::cast_from(0.19274729307122665472e-1_f64) * t102219 + F::cast_from(0.51405703062096148812e-1_f64) * t96240;
    (t102218, t102222)
}
