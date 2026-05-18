//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1012/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1012<F: Float>(t1756: F, t1784: F, t522: F, t535: F, t1789: F, t1792: F, t533: F, t1820: F, t110: F, t1803: F, t1808: F, t1811: F, t1821: F, t1825: F, t1826: F, t1828: F, t1829: F, t209: F, t22411: F, t508: F, t565: F, t571: F, t573: F, t580: F, t588: F, t622: F, t6383: F, t6388: F, t6395: F, t6399: F, t6401: F, t6424: F, t6428: F, t6466: F, t6500: F, t6504: F, t6511: F, t6821: F, t6825: F) -> (F, F, F, F) {
    let t22434 = F::new(0.4274e0) * t522 * t1756 * t1784 * t535;
    let t22439 = F::new(0.34366858576436911004e1) * t522 * t1789 * t1784 * t1792 * t533;
    let t22445 = t1820 * t1820;
    let t22485 = -F::new(0.66091990993888710196e1) * t522 * t1825 * t1820 * t1828 * t571 + F::new(0.13012297059337829057e0) * t522 * t6825 - t22434 + t22439 - F::new(0.1926377843805564792e1) * t522 * t6821 + F::new(0.11579285944033451271e4) * t6388 * t22411 * t1828 + F::new(0.96494049533612093922e2) * t1826 * t22445 * t1828 - F::new(0.67471169937307261776e-1) * t209 * t622 * t580 * t588 - F::new(0.41096e0) * t209 * t6511 * t6395 - F::new(0.13012297059337829057e0) * t209 * t6500 * t6466 - F::new(0.21309037037037037036e0) * t209 * t622 * t565 * t573 - F::new(0.27397333333333333333e0) * t209 * t110 * t1808 * t1811 + F::new(0.13218398198777742039e2) * t209 * t508 * t6399 * t6401 + F::new(0.4406132732925914013e1) * t209 * t110 * t1825 * t1829 - F::new(0.68493333333333333332e-1) * t209 * t1803 * t6383 - F::new(0.38024868119570572865e2) * t209 * t508 * t6424 * t6428 + F::new(0.13698666666666666666e0) * t209 * t6504 * t1821;
    (t22434, t22439, t22445, t22485)
}
