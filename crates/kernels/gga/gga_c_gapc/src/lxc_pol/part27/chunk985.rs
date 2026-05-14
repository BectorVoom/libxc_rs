//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 985/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk985<F: Float>(t15699: F, t7502: F, t9895: F, t15680: F, t26597: F, t7259: F, t11986: F, t3367: F, t6182: F, t33893: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F) -> (F,) {
    let t33917 = t9895 * t7502 * t15699;
    let t33920 = t7259 * t26597 * t15680;
    let t33923 = t11986 * t3367 * t6182;
    let t33925 = 0.12299149124710648149e-6 * t33893 - 0.29182498846122755858e-8 * t33897 - 0.10551281119038438161e-7 * t33899 + 0.1374296967252737644e-6 * t33902 - 0.18326250058315256483e-6 * t33904 + 0.6746961805555555556e-5 * t33908 - 0.10120442708333333334e-3 * t33911 + 0.61454016367594401047e-9 * t33914 - 0.23713668668337477784e-9 * t33917 + 0.252977417353824213e-7 * t33920 + 0.34752370105806885418e-3 * t33923;
    (t33925,)
}
