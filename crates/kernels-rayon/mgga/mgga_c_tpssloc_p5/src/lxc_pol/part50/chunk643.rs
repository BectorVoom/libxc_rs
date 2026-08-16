//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 643/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk643(t1340: f64, t5234: f64, t1358: f64, t1815: f64, t1362: f64, t242: f64, t3788: f64, t1336: f64, t557: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64) {
    let t5235 = t5234 * t1340;
    let t5238 = t1815 * t1358;
    let t5240 = t5234 * t1362;
    let t5245 = t3788 * t242;
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    (t5235, t5238, t5240, t5246, t5248)
}
