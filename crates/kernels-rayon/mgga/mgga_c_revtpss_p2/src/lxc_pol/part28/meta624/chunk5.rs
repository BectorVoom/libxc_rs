//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2219/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2219(t15682: f64, t25517: f64, t15811: f64, t16040: f64, t16078: f64, t25522: f64, t25569: f64, t25577: f64, t25580: f64, t4803: f64, t4808: f64, t93743: f64, t93745: f64, t93750: f64, t93755: f64) -> f64 {
    let t100240 = 0.3811023832717309953e-3_f64 * t25517 * t15682;
    let t100254 = 11.0_f64 / 324.0_f64 * t93743 + t93745 / 81.0_f64 + t93750 - 0.85748036236139473944e-3_f64 * t25580 * t16040 + t100240 - 0.57165357490759649296e-3_f64 * t93755 - 0.42874018118069736972e-3_f64 * t25580 * t16078 - 0.28582678745379824648e-3_f64 * t25522 * t15811 + 0.60976381323476959249e-2_f64 * t25577 * t4803 - 0.5081365110289746604e-2_f64 * t25577 * t4808 - 0.11433071498151929859e-2_f64 * t25569 * t4803 + 0.95275595817932748826e-3_f64 * t25569 * t4808;
    t100254
}
