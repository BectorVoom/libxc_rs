//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 816/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk816<F: Float>(t1662: F, t4972: F, t2894: F, t6521: F, t9938: F, t991: F, t1003: F, t6326: F, t9933: F, t18653: F, t4939: F, t14492: F, t18648: F, t14497: F, t18657: F, t330: F, t6539: F) -> (F, F, F, F, F, F, F) {
    let t19189 = t1662 * t4972;
    let t19190 = t2894 * t19189;
    let t19193 = t9938 * t6521;
    let t19194 = t991 * t19193;
    let t19196 = t6326 * t1003;
    let t19197 = t9933 * t19196;
    let t19200 = t4939 * t18653;
    let t19203 = t14492 * t18648;
    let t19206 = t14497 * t18657;
    let t19209 = t6539 * t330;
    (t19190, t19194, t19197, t19200, t19203, t19206, t19209)
}
