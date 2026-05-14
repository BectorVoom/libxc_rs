//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 664/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk664<F: Float>(t32094: F, t379: F, t7824: F, t5674: F, t32077: F, t8270: F, t1317: F, t28: F, t1800: F, t32082: F, t473: F, t7211: F, t469: F, t5665: F, t32061: F, t32066: F, t32072: F, t32080: F, t32085: F, t32089: F, t32093: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32096 = t7824 * t32094 * t379;
    let t32097 = t5674 * t32096;
    let t32099 = t8270 * t32077;
    let t32101 = t1317 * t28 * t32099;
    let t32102 = t1800 * t32082;
    let t32104 = t1317 * t28 * t32102;
    let t32106 = t7211 * t473;
    let t32107 = t469 * t32106;
    let t32109 = t5665 * t28 * t32107;
    let t32111 = t32061 / 2.0 + t32066 + 2.0 / 9.0 * t32072 + 4.0 / 3.0 * t32080 - 2.0 / 3.0 * t32085 - t32089 / 6.0 - t32093 - t32097 / 9.0 - t32101 + 2.0 / 3.0 * t32104 + t32109 / 12.0;
    (t32096, t32097, t32099, t32101, t32102, t32104, t32107, t32109, t32111)
}
