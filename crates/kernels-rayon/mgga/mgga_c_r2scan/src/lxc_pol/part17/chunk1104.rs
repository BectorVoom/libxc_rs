//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1104/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1104(t146: f64, t2206: f64, t2832: f64, t37848: f64, t37851: f64, t10810: f64, t1592: f64, t8156: f64, t10743: f64, t2699: f64, t37890: f64, t924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39745 = t146 * t2206 * t2832;
    let t39752 = 0.84755945902752848174e0_f64 * t37848;
    let t39753 = 0.25426783770825854452e1_f64 * t37851;
    let t39762 = t1592 * t10810 * t8156;
    let t39770 = t10743 * t2699;
    let t39772 = t37890 * t924;
    (t39745, t39752, t39753, t39762, t39770, t39772)
}
