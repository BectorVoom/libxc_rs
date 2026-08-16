//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1427/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1427(t12042: f64, t15642: f64, t17728: f64, t17872: f64, t27074: f64, t3116: f64, t3132: f64, t3133: f64, t35577: f64, t4386: f64, t4387: f64, t441: f64, t442: f64, t451: f64, t46539: f64, t46923: f64, t47001: f64, t5324: f64, t5333: f64, t54066: f64, t54079: f64, t54408: f64, t54430: f64, t54470: f64, t54472: f64, t54477: f64, t54509: f64, t54511: f64, t54518: f64, t58354: f64, t58365: f64, t58661: f64, t58942: f64) -> f64 {
    let t59611 = -154.0_f64 / 243.0_f64 * t54408 - 0.28977204965962526181e-1_f64 * t54430 - 0.63111673758367189645e-1_f64 * t46923 + 0.56076257758205259001e1_f64 * t441 * t442 * t58661 * t451 - t27074 - 0.9279375526865961238e3_f64 * t46539 * t5333 - 0.13735917720689745254e2_f64 * t3132 * t58942 * t3133 + 0.48295341609937543636e-1_f64 * t4386 * t12042 * t58354 + 0.12073835402484385909e-1_f64 * t4386 * t4387 * t58365 + 0.21464596271083352727e-2_f64 * t35577 - 0.1794440248262568288e1_f64 * t54470 - 0.18314556960919660338e2_f64 * t3132 * t54066 * t17728 + 0.94667510637550784468e-1_f64 * t3116 * t54079 * t5324 - 4.0_f64 / 81.0_f64 * t54472 + 0.24147670804968771818e-1_f64 * t54477 - 0.19318136643975017455e0_f64 * t15642 * t17872 - t54509 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t54511 + t47001 / 108.0_f64 - 0.12878757762650011637e0_f64 * t54518;
    t59611
}
