//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 683/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk683<F: Float>(t12041: F, t3193: F, t1851: F, t358: F, t1853: F, t920: F, t1909: F, t3114: F, t8506: F, t11593: F, t11999: F, t12002: F, t12005: F, t12009: F, t12013: F, t12017: F, t12022: F, t12027: F, t12030: F, t12035: F, t12038: F, t1901: F, t8567: F) -> (F,) {
    let t12042 = t3193 * t12041;
    let t12045 = t1851 * t358;
    let t12046 = t920 * t1853;
    let t12047 = t12045 * t12046;
    let t12048 = t1909 * t12047;
    let t12051 = t8506 * t3114;
    let t12055 = -t11999 + 22.0 / 27.0 * t12002 + 2.0 / 27.0 * t1901 * t12005 + t1901 * t12009 / 9.0 + 2.0 / 27.0 * t1901 * t12013 + 2.0 / 9.0 * t1901 * t12017 + 4.0 / 9.0 * t1901 * t12022 + 2.0 / 9.0 * t1901 * t12027 + 2.0 / 9.0 * t1901 * t12030 + 2.0 / 9.0 * t1901 * t12035 + 2.0 / 9.0 * t1901 * t12038 - 8.0 / 27.0 * t11593 * t12042 - 2.0 / 9.0 * t1901 * t12048 + 2.0 / 9.0 * t1901 * t12051 + 2.0 / 27.0 * t8567;
    (t12055,)
}
