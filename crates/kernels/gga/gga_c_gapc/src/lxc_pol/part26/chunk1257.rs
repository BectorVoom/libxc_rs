//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1257/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1257<F: Float>(t3646: F, t8489: F, t1464: F, t3651: F, t4059: F, t11248: F, t1444: F, t4855: F, t25042: F, t4050: F, t15260: F, t3948: F) -> (F, F, F, F, F) {
    let t35533 = t8489 * t3646;
    let t35536 = t3651 * t4059 * t1464;
    let t35539 = t11248 * t1444 * t4855;
    let t35541 = t25042 * t4050;
    let t35543 = t35541 * t3948 * t15260;
    (t35533, t35536, t35539, t35541, t35543)
}
