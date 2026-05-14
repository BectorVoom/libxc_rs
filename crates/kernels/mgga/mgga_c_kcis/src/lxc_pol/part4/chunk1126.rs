//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1126/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1126<F: Float>(t16217: F, t250: F, t3106: F, t11608: F, t11609: F, t16195: F, t16198: F, t16201: F, t16204: F, t16207: F, t16210: F, t16213: F, t16215: F, t11475: F, t16046: F, t16052: F, t16057: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16127: F, t16129: F, t16132: F, t16135: F, t16137: F, t16142: F, t16145: F, t16146: F, t16160: F, t16163: F, t16165: F, t16168: F) -> (F, F) {
    let t16219 = t250 * t3106 * t16217;
    let t16221 = 0.1898925e1 * t16195 + 0.16431333333333333333e0 * t16198 - 0.49293999999999999999e0 * t16201 - 0.27385555555555555556e-1 * t16204 - 0.36514074074074074075e-1 * t16207 + 0.10954222222222222222e0 * t16210 + 0.16431333333333333333e0 * t16213 + 0.3071625e0 * t16215 - t11608 - t11609 + 0.16431333333333333333e0 * t16219;
    let t16223 = -0.71202444444444444443e0 * t16127 - 0.91285185185185185185e-1 * t16129 - 0.76790625e-1 * t16132 - 0.1898925e1 * t16135 - 0.9494625e0 * t16137 - 0.21924222222222222222e1 * t16052 - 0.13287407407407407408e0 * t16046 - 0.65725333333333333332e0 * t16142 - t16145 + 0.36514074074074074074e-1 * t16146 + t16160 + 0.3071625e0 * t16163 + 0.15358125e0 * t16165 + 0.142419375e1 * t16168 - 0.33218518518518518518e0 * t16057 + 0.79724444444444444445e0 * t16067 - 0.19931111111111111111e0 * t16071 - 0.17938e1 * t16075 - 0.23917333333333333334e1 * t16080 + 0.59793333333333333334e0 * t16084 - 0.10954222222222222222e0 * t11475 + t16221;
    (t16219, t16223)
}
