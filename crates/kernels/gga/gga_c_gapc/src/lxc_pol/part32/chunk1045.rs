//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1045/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1045<F: Float>(t1743: F, t33219: F, t5703: F, t11451: F, t11518: F, t1690: F, t11326: F, t25871: F, t1030: F, t25876: F, t34073: F, t1040: F, t34681: F, t26396: F, t34058: F, t11546: F, t424: F, t641: F) -> (F, F, F, F, F, F, F) {
    let t34881 = t1743 * t33219 * t5703;
    let t34884 = t11518 * t11451 * t1690;
    let t34886 = t11326 * t25871;
    let t34889 = t1030 * t34073 * t25876;
    let t34891 = t34681 * t1040;
    let t34894 = t1030 * t34058 * t26396;
    let t34897 = t424 * t641 * t11546;
    (t34881, t34884, t34886, t34889, t34891, t34894, t34897)
}
