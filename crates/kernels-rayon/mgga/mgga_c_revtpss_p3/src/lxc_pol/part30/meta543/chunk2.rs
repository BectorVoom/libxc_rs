//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1977/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1977(t5: f64, t29387: f64, t29419: f64, t117: f64, t1310: f64, t1843: f64, t2127: f64, t27136: f64, t27139: f64, t27152: f64, t27156: f64, t27834: f64, t27835: f64, t28022: f64, t28045: f64, t28058: f64, t28060: f64, t508: f64, t5517: f64, t649: f64, t7584: f64, t8152: f64, t8233: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t29421 = piecewise3(t8, 0.0_f64, t29387 + t29419);
    let t29422 = t29421 * t117;
    let t29425 = -t1310 * t8152 - t1843 * t7584 - t2127 * t5517 - t29422 * t508 - t649 * t8233 - t27136 - t27139 + t27152 - t27156 + t27834 + t27835 + t28022 - t28045 - t28058 - t28060;
    (t29421, t29422, t29425)
}
