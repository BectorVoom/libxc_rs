//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1203/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1203(t265: f64, t502: f64, t30865: f64, t30922: f64, t1300: f64, t1832: f64, t198: f64, t27041: f64, t29317: f64, t29930: f64, t336: f64, t5023: f64, t6748: f64, t6752: f64, t7673: f64) -> (f64, f64) {
    let t503 = t265 < t502;
    let t30923 = t30865 + t30922;
    let t30936 = piecewise3(t503, t1300 * t198 * t30923 * t336 - 2.0_f64 * t1832 * t29317 * t5023 + 2.0_f64 * t27041 * t5023 * t6752 - t5023 * t6748 * t7673, t29930);
    (t30923, t30936)
}
