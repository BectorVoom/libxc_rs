//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 863/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk863<F: Float>(t3375: F, t9673: F, t320: F, t8700: F, t3379: F, t3402: F, t8838: F, t3406: F, t7115: F, t9921: F, t2598: F, t3404: F) -> (F, F, F, F, F) {
    let t10024 = t9673 * t3375;
    let t10026 = t320 * t8700;
    let t10027 = t10026 * t3379;
    let t10029 = t3402 * t8838;
    let t10030 = t7115 * t3406;
    let t10031 = t9921 * t10030;
    let t10032 = t10029 * t10031;
    let t10034 = t3404 * t2598;
    (t10024, t10027, t10031, t10032, t10034)
}
