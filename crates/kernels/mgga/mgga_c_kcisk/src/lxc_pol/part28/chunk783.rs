//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 783/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk783<F: Float>(t9708: F, t9709: F, t4581: F, t748: F, t1873: F, t1950: F, t1800: F, t1954: F, t9702: F, t9706: F) -> (F, F, F, F, F) {
    let t9710 = t9708 * t9709;
    let t9712 = t4581 * t748;
    let t9714 = t1873 * t1950;
    let t9716 = t1800 * t1954;
    let t9718 = t9702 / 16.0 - t9706 / 16.0 + t9710 / 24.0 - t9712 / 128.0 + t9714 / 128.0 - t9716 / 96.0;
    (t9710, t9712, t9714, t9716, t9718)
}
