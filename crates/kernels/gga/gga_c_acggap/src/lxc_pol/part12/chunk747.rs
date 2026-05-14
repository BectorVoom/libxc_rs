//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 747/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk747<F: Float>(t599: F, t8791: F, t1181: F, t7413: F, t1165: F, t604: F, t8406: F, t7346: F, t8901: F, t7337: F, t360: F, t525: F, t2068: F, t7351: F, t8906: F, t8402: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8947 = t599 * t8791;
    let t8948 = t1181 * t8947;
    let t8949 = t7413 * t8948;
    let t8952 = t1165 * t604 * t8406;
    let t8953 = t7346 * t8952;
    let t8956 = t1165 * t604 * t8901;
    let t8957 = t7337 * t8956;
    let t8960 = t525 * t360;
    let t8962 = t1181 * t604 * t8960;
    let t8963 = t2068 * t8962;
    let t8966 = t1165 * t7351 * t8906;
    let t8967 = t2068 * t8966;
    let t8970 = t1165 * t604 * t8402;
    (t8947, t8948, t8949, t8952, t8953, t8956, t8957, t8960, t8962, t8963, t8966, t8967, t8970)
}
