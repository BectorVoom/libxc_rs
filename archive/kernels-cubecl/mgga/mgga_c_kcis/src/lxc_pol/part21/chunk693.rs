//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 693/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk693<F: Float>(t441: F, t914: F, t1295: F, t235: F, t2209: F, t915: F, t1250: F, t3049: F) -> (F, F, F, F) {
    let t7676 = t914 * t441;
    let t7679 = t235 * t1295;
    let t7684 = t915 * t2209;
    let t7687 = t3049 * t1250;
    (t7676, t7679, t7684, t7687)
}
