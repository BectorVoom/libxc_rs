//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1270/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1270<F: Float>(t12042: F, t15642: F, t17728: F, t17872: F, t27074: F, t3116: F, t3132: F, t3133: F, t35577: F, t4386: F, t4387: F, t441: F, t442: F, t451: F, t46539: F, t46923: F, t47001: F, t5324: F, t5333: F, t54066: F, t54079: F, t54408: F, t54430: F, t54470: F, t54472: F, t54477: F, t54509: F, t54511: F, t54518: F, t58354: F, t58365: F, t58661: F, t58942: F) -> (F,) {
    let t59611 = -154.0 / 243.0 * t54408 - 0.28977204965962526181e-1 * t54430 - 0.63111673758367189645e-1 * t46923 + 0.56076257758205259001e1 * t441 * t442 * t58661 * t451 - t27074 - 0.9279375526865961238e3 * t46539 * t5333 - 0.13735917720689745254e2 * t3132 * t58942 * t3133 + 0.48295341609937543636e-1 * t4386 * t12042 * t58354 + 0.12073835402484385909e-1 * t4386 * t4387 * t58365 + 0.21464596271083352727e-2 * t35577 - 0.1794440248262568288e1 * t54470 - 0.18314556960919660338e2 * t3132 * t54066 * t17728 + 0.94667510637550784468e-1 * t3116 * t54079 * t5324 - 4.0 / 81.0 * t54472 + 0.24147670804968771818e-1 * t54477 - 0.19318136643975017455e0 * t15642 * t17872 - t54509 / 27.0 + 2.0 / 27.0 * t54511 + t47001 / 108.0 - 0.12878757762650011637e0 * t54518;
    (t59611,)
}
