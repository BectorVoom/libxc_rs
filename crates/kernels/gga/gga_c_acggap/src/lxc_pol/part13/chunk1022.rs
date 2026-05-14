//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1022/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1022<F: Float>(t1488: F, t2060: F, t2061: F, t1165: F, t20817: F, t604: F, t7337: F, t4975: F, t7561: F, t5157: F, t22401: F, t7351: F, t7413: F, t30817: F, t8948: F, t8793: F) -> (F, F, F, F, F, F, F) {
    let t35860 = t2060 * t1488 * t2061;
    let t35864 = t7337 * t1165 * t604 * t20817;
    let t35866 = t7561 * t4975;
    let t35868 = t7561 * t5157;
    let t35872 = t7413 * t1165 * t7351 * t22401;
    let t35874 = t30817 * t8948;
    let t35875 = 0.25724410870841842184e-2 * t35874;
    let t35876 = t30817 * t8793;
    (t35860, t35864, t35866, t35868, t35872, t35875, t35876)
}
