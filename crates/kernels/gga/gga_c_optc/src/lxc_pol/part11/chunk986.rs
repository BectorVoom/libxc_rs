//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 986/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk986<F: Float>(t4477: F, t5110: F, t3245: F, t17907: F, t914: F, t17903: F, t5311: F, t9073: F, t4327: F, t4356: F, t4458: F, t17697: F, t430: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18030 = t5110 * t4477;
    let t18031 = t3245 * t18030;
    let t18034 = t914 * t17907;
    let t18037 = t914 * t17903;
    let t18042 = t9073 * t5311;
    let t18043 = t4327 * t18042;
    let t18054 = t4356 * t5311;
    let t18055 = t4458 * t18054;
    let t18058 = t430 * t17697;
    (t18030, t18031, t18034, t18037, t18042, t18043, t18054, t18055, t18058)
}
