//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1012/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1012(t1756: f64, t1784: f64, t522: f64, t535: f64, t1789: f64, t1792: f64, t533: f64, t1820: f64, t110: f64, t1803: f64, t1808: f64, t1811: f64, t1821: f64, t1825: f64, t1826: f64, t1828: f64, t1829: f64, t209: f64, t22411: f64, t508: f64, t565: f64, t571: f64, t573: f64, t580: f64, t588: f64, t622: f64, t6383: f64, t6388: f64, t6395: f64, t6399: f64, t6401: f64, t6424: f64, t6428: f64, t6466: f64, t6500: f64, t6504: f64, t6511: f64, t6821: f64, t6825: f64) -> (f64, f64, f64, f64) {
    let t22434 = 0.4274e0_f64 * t522 * t1756 * t1784 * t535;
    let t22439 = 0.34366858576436911004e1_f64 * t522 * t1789 * t1784 * t1792 * t533;
    let t22445 = t1820 * t1820;
    let t22485 = -0.66091990993888710196e1_f64 * t522 * t1825 * t1820 * t1828 * t571 + 0.13012297059337829057e0_f64 * t522 * t6825 - t22434 + t22439 - 0.1926377843805564792e1_f64 * t522 * t6821 + 0.11579285944033451271e4_f64 * t6388 * t22411 * t1828 + 0.96494049533612093922e2_f64 * t1826 * t22445 * t1828 - 0.67471169937307261776e-1_f64 * t209 * t622 * t580 * t588 - 0.41096e0_f64 * t209 * t6511 * t6395 - 0.13012297059337829057e0_f64 * t209 * t6500 * t6466 - 0.21309037037037037036e0_f64 * t209 * t622 * t565 * t573 - 0.27397333333333333333e0_f64 * t209 * t110 * t1808 * t1811 + 0.13218398198777742039e2_f64 * t209 * t508 * t6399 * t6401 + 0.4406132732925914013e1_f64 * t209 * t110 * t1825 * t1829 - 0.68493333333333333332e-1_f64 * t209 * t1803 * t6383 - 0.38024868119570572865e2_f64 * t209 * t508 * t6424 * t6428 + 0.13698666666666666666e0_f64 * t209 * t6504 * t1821;
    (t22434, t22439, t22445, t22485)
}
