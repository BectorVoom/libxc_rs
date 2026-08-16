//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1703/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703(t1923: f64, t26205: f64, t2048: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t25150: f64, t25159: f64, t25162: f64, t26170: f64, t26172: f64, t26175: f64, t26180: f64, t26182: f64, t26185: f64, t26187: f64, t26190: f64, t6954: f64, t6960: f64, t6963: f64, t7343: f64, t7352: f64) -> (f64, f64) {
    let t26207 = 88.0_f64 / 27.0_f64 * t1923 * t26205;
    let t26208 = t25150 * t2048 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6954 * t7352 - 16.0_f64 / 9.0_f64 * t26170 + t1923 * t26172 / 3.0_f64 + 10.0_f64 * t26175 * t25159 + 80.0_f64 / 9.0_f64 * t26180 + 20.0_f64 / 3.0_f64 * t25162 * t26182 + 32.0_f64 / 9.0_f64 * t26185 - 10.0_f64 / 3.0_f64 * t26187 * t6960 - 16.0_f64 / 9.0_f64 * t26190 - 4.0_f64 / 3.0_f64 * t25102 * t2048 - 10.0_f64 / 3.0_f64 * t7343 * t25110 - 5.0_f64 / 3.0_f64 * t7343 * t25114 - 2.0_f64 / 3.0_f64 * t25117 * t2048 - 2.0_f64 / 3.0_f64 * t25120 * t2048 - 4.0_f64 / 3.0_f64 * t6963 * t7352 + t26207;
    (t26207, t26208)
}
