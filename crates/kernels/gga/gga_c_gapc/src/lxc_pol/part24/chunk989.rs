//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 989/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk989<F: Float>(t11597: F, t9574: F, t9578: F, t11917: F, t3363: F, t9846: F, t11902: F, t15938: F, t11990: F, t19139: F, t2597: F, t1: F, t33543: F, t1084: F, t33961: F, t11311: F, t11791: F, t2520: F) -> (F, F, F, F, F, F, F) {
    let t34095 = t9574 * t11597 * t9578;
    let t34098 = t3363 * t11917 * t9846;
    let t34100 = t11902 * t15938;
    let t34104 = t11990 * t2597 * t19139;
    let t34106 = t33543 * t1;
    let t34108 = t1084 * t34106 * t33961;
    let t34111 = t2520 * t11311 * t11791;
    (t34095, t34098, t34100, t34104, t34106, t34108, t34111)
}
