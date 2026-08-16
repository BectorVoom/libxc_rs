//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1122/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1122(t33893: f64, t33897: f64, t33899: f64, t33902: f64, t33904: f64, t33908: f64, t33911: f64, t33914: f64, t33917: f64, t33920: f64, t33923: f64, t2268: f64, t3438: f64, t3439: f64) -> (f64, f64) {
    let t33925 = 0.12299149124710648149e-6_f64 * t33893 - 0.29182498846122755858e-8_f64 * t33897 - 0.10551281119038438161e-7_f64 * t33899 + 0.1374296967252737644e-6_f64 * t33902 - 0.18326250058315256483e-6_f64 * t33904 + 0.6746961805555555556e-5_f64 * t33908 - 0.10120442708333333334e-3_f64 * t33911 + 0.61454016367594401047e-9_f64 * t33914 - 0.23713668668337477784e-9_f64 * t33917 + 0.252977417353824213e-7_f64 * t33920 + 0.34752370105806885418e-3_f64 * t33923;
    let t33928 = t3438 * t2268 * t3439;
    (t33925, t33928)
}
