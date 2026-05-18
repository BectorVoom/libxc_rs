//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1122/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1122<F: Float>(t33893: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F, t33917: F, t33920: F, t33923: F, t2268: F, t3438: F, t3439: F) -> (F, F) {
    let t33925 = F::new(0.12299149124710648149e-6) * t33893 - F::new(0.29182498846122755858e-8) * t33897 - F::new(0.10551281119038438161e-7) * t33899 + F::new(0.1374296967252737644e-6) * t33902 - F::new(0.18326250058315256483e-6) * t33904 + F::new(0.6746961805555555556e-5) * t33908 - F::new(0.10120442708333333334e-3) * t33911 + F::new(0.61454016367594401047e-9) * t33914 - F::new(0.23713668668337477784e-9) * t33917 + F::new(0.252977417353824213e-7) * t33920 + F::new(0.34752370105806885418e-3) * t33923;
    let t33928 = t3438 * t2268 * t3439;
    (t33925, t33928)
}
