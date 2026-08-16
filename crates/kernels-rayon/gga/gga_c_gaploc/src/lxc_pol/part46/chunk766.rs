//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 766/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk766(t1381: f64, t3362: f64, t1959: f64, t3455: f64, t10795: f64, t747: f64, t1: f64, t10215: f64, t106: f64, t192: f64, t10496: f64, t540: f64) -> (f64, f64, f64, f64, f64) {
    let t33959 = t3362 * t1381;
    let t33992 = t3455 * t1959;
    let t34013 = t10795 * t747;
    let t34131 = t10215 * t1 * t106 * t192;
    let t34157 = t10496 * t540;
    (t33959, t33992, t34013, t34131, t34157)
}
