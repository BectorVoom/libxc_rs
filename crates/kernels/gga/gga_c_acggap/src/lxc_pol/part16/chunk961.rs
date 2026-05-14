//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 961/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk961<F: Float>(t7839: F, t9758: F, t2068: F, t4680: F, t9757: F, t30268: F, t9597: F, t1181: F, t25742: F, t604: F, t1165: F, t26459: F, t7337: F, t7315: F, t9622: F, t2016: F, t9626: F) -> (F, F, F, F, F, F, F) {
    let t39026 = t7839 * t9758;
    let t39029 = t2068 * t4680 * t9757;
    let t39031 = t30268 * t9597;
    let t39035 = t2068 * t1181 * t604 * t25742;
    let t39039 = t7337 * t1165 * t604 * t26459;
    let t39041 = t7315 * t9622;
    let t39043 = t2016 * t9626;
    (t39026, t39029, t39031, t39035, t39039, t39041, t39043)
}
