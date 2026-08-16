//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1350/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1350(t3399: f64, t4637: f64, t10452: f64, t10488: f64, t1328: f64, t1445: f64, t1450: f64, t31828: f64, t34025: f64, t34032: f64, t34036: f64, t34038: f64, t34045: f64, t34052: f64, t34054: f64, t34056: f64, t34058: f64, t34061: f64, t34065: f64, t447: f64, t4527: f64, t4614: f64, t4771: f64, t574: f64) -> f64 {
    let t34067 = 0.11502877786176224903e2_f64 * t4637 * t3399;
    let t34068 = -t34025 - 0.12269736305254639896e2_f64 * t574 * t4614 * t10488 - t34032 - t34036 - t34038 - 0.46011511144704899612e1_f64 * t4771 * t10452 - 0.46011511144704899612e1_f64 * t1450 * t1445 * t31828 * t447 + 0.27606906686822939767e2_f64 * t4527 * t1445 * t34045 * t1328 - t34052 - t34054 - t34056 + t34058 + t34061 - t34065 + t34067;
    t34068
}
