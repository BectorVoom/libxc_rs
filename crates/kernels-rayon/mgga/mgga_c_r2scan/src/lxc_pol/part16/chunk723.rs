//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 723/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk723(t5865: f64, t5866: f64, t160: f64, t35: f64, t164: f64, t1774: f64, t604: f64, t1780: f64, t601: f64, t2099: f64, t161: f64, t2036: f64, t406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5868 = 0.10526802520742363173e2_f64 * t5865 * t5866;
    let t5869 = t160 * t35;
    let t5871 = 1320.0_f64 * t5869 * t164;
    let t5872 = t1774 * t604;
    let t5874 = t601 * t1780;
    let t5876 = 1.0_f64 / t2099;
    let t5878 = 2184.0_f64 * t161 * t5876;
    let t5883 = t406 * t2036;
    (t5868, t5871, t5872, t5874, t5878, t5883)
}
