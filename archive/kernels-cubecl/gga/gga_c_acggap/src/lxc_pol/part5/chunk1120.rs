//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1120/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1120<F: Float>(t1711: F, t715: F, t11893: F, t15050: F, t18217: F, t6024: F, t807: F, t11910: F, t18222: F, t11916: F, t11922: F, t11898: F, t11900: F, t11906: F, t11909: F, t11914: F, t11921: F, t11938: F, t11944: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20030 = t715 * t1711;
    let t20031 = F::cast_from(32.0_f64) * t20030;
    let t20032 = F::cast_from(0.70178683471615754484e1_f64) * t11893;
    let t20033 = F::cast_from(2.0_f64) * t15050;
    let t20034 = F::cast_from(8.0_f64) * t18217;
    let t20035 = t6024 * t807;
    let t20036 = F::cast_from(0.24415263074675393405e-3_f64) * t20035;
    let t20037 = F::cast_from(120.0_f64) * t11910;
    let t20038 = F::cast_from(120.0_f64) * t18222;
    let t20039 = F::cast_from(480.0_f64) * t11916;
    let t20040 = F::cast_from(0.11696447245269292414e1_f64) * t11922;
    let t20041 = -t20031 + t20032 + t11898 + t20033 + t11900 + t11906 + t20034 - t11909 + t20036 + t20037 - t11914 + t20038 - t20039 - t11921 - t20040 - t11938 - t11944;
    (t20031, t20032, t20033, t20034, t20036, t20037, t20038, t20039, t20040, t20041)
}
