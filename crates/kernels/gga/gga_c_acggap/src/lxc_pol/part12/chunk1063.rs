//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1063/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1063<F: Float>(t36096: F, t36115: F, t36125: F, t36129: F, t36131: F, t36133: F, t36135: F, t36139: F, t31752: F, t32923: F, t36100: F, t36103: F, t36107: F, t36111: F, t36119: F, t36123: F, t36127: F, t36137: F) -> (F,) {
    let t37864 = 0.62896184579208304138e-3 * t36096;
    let t37869 = 0.42874018118069736972e-3 * t36115;
    let t37872 = 0.32012600194825403606e-1 * t36125;
    let t37874 = 0.42874018118069736972e-3 * t36129;
    let t37875 = 0.85748036236139473944e-3 * t36131;
    let t37876 = 0.85748036236139473944e-3 * t36133;
    let t37877 = 0.57165357490759649296e-3 * t36135;
    let t37879 = 0.32012600194825403606e-1 * t36139;
    let t37881 = t37864 + 0.31448092289604152069e-3 * t36100 + 0.21437009059034868486e-2 * t36103 + 0.21437009059034868486e-2 * t36107 + 0.10718504529517434243e-2 * t36111 - t37869 + 0.83861579438944405517e-3 * t36119 - 0.12579236915841660828e-2 * t36123 + t37872 + 0.75475421495049964966e-2 * t36127 - t37874 - t37875 - t37876 - t37877 + 0.39624596284901231607e-1 * t36137 - t37879 - t32923 - 0.52832795046534975475e-1 * t31752;
    (t37881,)
}
