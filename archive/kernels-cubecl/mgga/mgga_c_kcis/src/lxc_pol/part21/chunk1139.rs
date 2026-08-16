//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1139/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1139<F: Float>(t1250: F, t14654: F, t167: F, t7704: F, t14554: F, t1003: F, t4781: F, t26686: F) -> (F, F, F, F, F) {
    let t27895 = t14654 * t1250;
    let t27903 = t7704 * t167;
    let t27904 = t14554 * t27903;
    let t27910 = t4781 * t1003;
    let t27911 = t26686 * t27910;
    (t27895, t27903, t27904, t27910, t27911)
}
