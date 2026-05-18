//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1258/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1258<F: Float>(t1040: F, t34681: F, t1030: F, t26396: F, t34058: F, t11546: F, t424: F, t641: F, t655: F, t1266: F, t3696: F, t3703: F) -> (F, F, F, F, F) {
    let t34891 = t34681 * t1040;
    let t34894 = t1030 * t34058 * t26396;
    let t34897 = t424 * t641 * t11546;
    let t34900 = t424 * t655 * t11546;
    let t34905 = t1266 * t3696 * t3703;
    (t34891, t34894, t34897, t34900, t34905)
}
