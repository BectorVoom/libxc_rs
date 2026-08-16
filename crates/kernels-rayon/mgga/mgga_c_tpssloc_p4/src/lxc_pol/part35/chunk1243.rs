//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1243/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1243(t29506: f64, t29864: f64, t3: f64, t1458: f64, t24972: f64, t27921: f64, t28888: f64, t28890: f64, t28892: f64, t28895: f64, t28898: f64, t28901: f64, t28903: f64, t5456: f64, t5493: f64, t577: f64, t7423: f64) -> (f64, f64, f64) {
    let t29865 = t29506 + t29864;
    let t29866 = t3 * t29865;
    let t29884 = 0.45e1_f64 * t29865 * t577 + 27.0_f64 * t27921 * t1458 + 27.0_f64 * t24972 * t5456 + 0.135e2_f64 * t7423 * t5493 + t28888 + t28890 + t28892 + t28895 + t28898 + t28901 + t28903;
    (t29865, t29866, t29884)
}
