//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 992/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk992<F: Float>(t142: F, t6289: F, t7436: F, t6300: F, t6309: F, t6313: F, t8888: F, t2020: F, t9761: F, t7839: F, t9633: F, t2068: F, t2263: F, t35137: F, t8480: F, t8521: F) -> (F, F, F, F, F, F, F, F) {
    let t39514 = t7436 * t142 * t6289;
    let t39517 = t7436 * t142 * t6300;
    let t39520 = t7436 * t142 * t6309;
    let t39525 = t8888 * t142 * t6313;
    let t39527 = t2020 * t9761;
    let t39534 = t7839 * t9633;
    let t39537 = t2068 * t35137 * t2263;
    let t39540 = t2068 * t8480 * t8521;
    (t39514, t39517, t39520, t39525, t39527, t39534, t39537, t39540)
}
