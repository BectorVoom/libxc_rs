//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1165/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1165(t104115: f64, t1936: f64, t111734: f64, t29427: f64, t7002: f64, t7334: f64, t8245: f64, t7331: f64, t7696: f64, t7953: f64, t7950: f64, t2170: f64, t28271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t129488 = t104115 * t1936;
    let t129489 = t111734 * t1936;
    let t129490 = t29427 * t7002;
    let t129541 = t8245 * t7334;
    let t129543 = t8245 * t7331;
    let t129555 = t7696 * t7953;
    let t129559 = t7696 * t7950;
    let t129562 = t2170 * t28271;
    (t129488, t129489, t129490, t129541, t129543, t129555, t129559, t129562)
}
