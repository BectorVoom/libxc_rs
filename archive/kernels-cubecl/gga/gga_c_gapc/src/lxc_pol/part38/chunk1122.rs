//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1122/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1122<F: Float>(t33893: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F, t33917: F, t33920: F, t33923: F, t2268: F, t3438: F, t3439: F) -> (F, F) {
    let t33925 = F::cast_from(0.12299149124710648149e-6_f64) * t33893 - F::cast_from(0.29182498846122755858e-8_f64) * t33897 - F::cast_from(0.10551281119038438161e-7_f64) * t33899 + F::cast_from(0.1374296967252737644e-6_f64) * t33902 - F::cast_from(0.18326250058315256483e-6_f64) * t33904 + F::cast_from(0.6746961805555555556e-5_f64) * t33908 - F::cast_from(0.10120442708333333334e-3_f64) * t33911 + F::cast_from(0.61454016367594401047e-9_f64) * t33914 - F::cast_from(0.23713668668337477784e-9_f64) * t33917 + F::cast_from(0.252977417353824213e-7_f64) * t33920 + F::cast_from(0.34752370105806885418e-3_f64) * t33923;
    let t33928 = t3438 * t2268 * t3439;
    (t33925, t33928)
}
