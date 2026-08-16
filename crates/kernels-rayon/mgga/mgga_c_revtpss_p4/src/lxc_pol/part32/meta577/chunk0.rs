//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1903/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1903(t2453: f64, t3908: f64, t8086: f64, t28829: f64, t689: f64, t25899: f64, t26271: f64, t27884: f64, t28862: f64, t686: f64, t72: f64, t25895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102266 = t2453 * t8086 * t3908;
    let t102268 = t28829 * t689;
    let t102270 = 0.25702851531048074406e-1_f64 * t25899 * t102268;
    let t102272 = 0.25702851531048074406e-1_f64 * t27884 * t26271;
    let t102274 = t28862 * t72 * t686;
    let t102276 = 0.28912093960683998208e-1_f64 * t25895 * t102274;
    (t102266, t102268, t102270, t102272, t102274, t102276)
}
