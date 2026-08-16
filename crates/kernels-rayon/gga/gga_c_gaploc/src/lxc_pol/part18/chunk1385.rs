//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1385/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1385(t10601: f64, t4372: f64, t107: f64, t31730: f64, t544: f64, t10392: f64, t17568: f64, t31557: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t34556 = 0.92686455430723328401e-1_f64 * t10601 * t4372;
    let t34558 = t544 * t31730 * t107;
    let t34566 = 0.15337170381568299871e1_f64 * t17568 * t10392;
    let t34567 = t31557 * t475;
    (t34556, t34558, t34566, t34567)
}
