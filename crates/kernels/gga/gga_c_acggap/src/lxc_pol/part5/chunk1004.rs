//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1004/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1004<F: Float>(t14967: F, t14969: F, t14972: F, t14984: F, t14986: F, t14999: F, t15003: F, t15005: F, t11829: F, t15008: F, t15010: F, t11775: F, t11778: F, t11780: F, t11792: F, t11825: F, t11828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19995 = 0.70178683471615754484e1 * t14967;
    let t19996 = 48.0 * t14969;
    let t19997 = 96.0 * t14972;
    let t19998 = 0.11696447245269292414e1 * t14984;
    let t19999 = 64.0 * t14986;
    let t20000 = 0.11696447245269292414e1 * t14999;
    let t20001 = 0.23392894490538584828e1 * t15003;
    let t20002 = 0.20508037716432813315e4 * t15005;
    let t20003 = 8.0 * t11829;
    let t20004 = 8.0 * t15008;
    let t20005 = 8.0 * t15010;
    let t20006 = -t11775 + t11778 - t11780 + t11792 + t11825 - t19995 - t19996 + t19997 - t19998 - t19999 + t11828 - t20000 - t20001 - t20002 - t20003 - t20004 - t20005;
    (t19995, t19996, t19997, t19998, t19999, t20000, t20001, t20002, t20003, t20004, t20005, t20006)
}
