//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 820/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk820(t5559: f64, t803: f64, t1705: f64, t806: f64, t935: f64, t937: f64) -> (f64, f64, f64, f64) {
    let t5560 = t5559 * t803;
    let t5567 = t1705 * t806;
    let t5568 = t5567 * t935;
    let t5570 = t935 * t937;
    (t5560, t5567, t5568, t5570)
}
