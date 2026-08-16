//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 650/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk650(t1485: f64, t2814: f64, t198: f64, t330: f64, t3764: f64, t3767: f64, t3769: f64, t3772: f64, t3809: f64, t3813: f64, t3891: f64, t3893: f64, t3896: f64, t3898: f64, t3902: f64, t3906: f64, t3911: f64, t4019: f64, t4023: f64, t993: f64, t995: f64) -> (f64, f64) {
    let t4024 = t1485 * t2814;
    let t4027 = t198 * t330 * t4019 * t995 - t4023 * t4024 * t993 - t3764 + t3767 + t3769 - t3772 + t3809 + t3813 + t3891 + t3893 - t3896 - t3898 + t3902 - t3906 - t3911;
    (t4024, t4027)
}
