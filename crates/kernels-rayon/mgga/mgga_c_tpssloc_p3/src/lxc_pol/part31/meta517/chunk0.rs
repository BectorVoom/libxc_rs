//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1715/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1715(t28951: f64, t510: f64, t2035: f64, t5456: f64, t28834: f64, t7170: f64, t2057: f64, t28241: f64, t1510: f64, t26661: f64, t24255: f64, t5585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28952 = t510 * t28951;
    let t28959 = t2035 * t5456;
    let t28969 = t7170 * t28834;
    let t28972 = t2057 * t28241;
    let t28997 = t26661 * t1510;
    let t29000 = t24255 * t5585;
    (t28952, t28959, t28969, t28972, t28997, t29000)
}
