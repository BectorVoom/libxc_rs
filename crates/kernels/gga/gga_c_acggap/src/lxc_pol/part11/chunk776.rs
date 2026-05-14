//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 776/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk776<F: Float>(t1416: F, t301: F, t322: F, t20432: F, t944: F, t5752: F, t943: F, t1454: F, t372: F, t1182: F, t1410: F, t1487: F, t407: F, t1539: F, t1439: F, t360: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22107 = t1416 * t301;
    let t22275 = t1416 * t322;
    let t22401 = t20432 * t944;
    let t22710 = t5752 * t943;
    let t22778 = t1454 * t372;
    let t23045 = t1182 * t1410;
    let t23445 = t407 * t1487;
    let t23688 = t1539 * t1410;
    let t23718 = t1439 * t360;
    (t22107, t22275, t22401, t22710, t22778, t23045, t23445, t23688, t23718)
}
