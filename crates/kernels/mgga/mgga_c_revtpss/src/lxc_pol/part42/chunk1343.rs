//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1343/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1343<F: Float>(t31653: F, t569: F, t1911: F, t8406: F, t1843: F, t2198: F, t6934: F, t6765: F, t508: F, t1312: F, t18245: F, t2199: F, t2201: F, t29508: F, t30138: F, t30143: F, t4248: F, t651: F, t7732: F, t7889: F, t8393: F, t8407: F, t8411: F, t8413: F) -> (F, F, F, F, F, F, F) {
    let t31654 = t31653 * t569;
    let t31657 = t8406 * t1911;
    let t31660 = t1843 * t8406;
    let t31663 = t2198 * t6934;
    let t31674 = t6765 * t2198;
    let t31677 = t508 * t31653;
    let t31700 = 2.0 * t1312 * t31654 + 4.0 * t1312 * t31657 + 2.0 * t1312 * t31663 - 2.0 * t18245 * t2199 + 2.0 * t18245 * t2201 - 2.0 * t2199 * t29508 - 4.0 * t2199 * t30138 + 4.0 * t2201 * t30138 + 2.0 * t2201 * t30143 - 4.0 * t31660 * t651 - 2.0 * t31674 * t651 - 2.0 * t31677 * t651 - 4.0 * t4248 * t8393 - 4.0 * t4248 * t8407 + 4.0 * t4248 * t8411 + 4.0 * t4248 * t8413 - 4.0 * t7732 * t8393 - 4.0 * t7732 * t8407 + 4.0 * t7889 * t8411 + 4.0 * t7889 * t8413;
    (t31654, t31657, t31660, t31663, t31674, t31677, t31700)
}
