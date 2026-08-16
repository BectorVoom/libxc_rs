//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 845/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk845(t1: f64, t106: f64, t10667: f64, t316: f64, t11000: f64, t783: f64, t1381: f64, t3362: f64, t1959: f64, t3455: f64, t10795: f64, t747: f64) -> (f64, f64, f64, f64, f64) {
    let t33725 = t10667 * t1 * t106 * t316;
    let t33778 = t11000 * t783;
    let t33959 = t3362 * t1381;
    let t33992 = t3455 * t1959;
    let t34013 = t10795 * t747;
    (t33725, t33778, t33959, t33992, t34013)
}
