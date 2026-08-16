//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1042/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1042(t1332: f64, t2013: f64, t22693: f64, t22707: f64, t26379: f64, t26381: f64, t26386: f64, t26390: f64, t26393: f64, t26398: f64, t26401: f64, t26404: f64, t26406: f64, t26412: f64, t26416: f64, t26419: f64, t26424: f64, t26427: f64, t26429: f64, t5230: f64, t5344: f64, t544: f64, t7747: f64) -> f64 {
    let t26431 = 0.16449340668482264365e-1_f64 * t26379 + 0.38381794893125283518e-1_f64 * t26381 - t22693 + t5230 * t2013 - 0.16449340668482264365e-1_f64 * t26386 - 0.16449340668482264365e-1_f64 * t26390 + 0.82246703342411321825e-2_f64 * t26393 - 0.16449340668482264365e-1_f64 * t26398 + t1332 * t7747 + t544 * t26401 - t5344 * t26404 + 0.19190897446562641759e-1_f64 * t26406 + 0.41123351671205660912e-2_f64 * t22707 - 0.82246703342411321825e-2_f64 * t26412 + 0.16449340668482264365e-1_f64 * t26416 - 0.82246703342411321825e-2_f64 * t26419 + 0.16449340668482264365e-1_f64 * t26424 + 0.41123351671205660912e-2_f64 * t26427 - 0.19190897446562641759e-1_f64 * t26429;
    t26431
}
