//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 704/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk704<F: Float>(t1650: F, t7909: F, t5709: F, t1938: F, t7914: F, t6176: F, t4163: F, t7923: F, t1394: F, t1982: F, t2243: F, t303: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8154 = t7909 * t1650;
    let t8155 = t5709 * t8154;
    let t8158 = t7914 * t1938;
    let t8159 = t6176 * t8158;
    let t8164 = t4163 * t1650;
    let t8165 = t7923 * t8164;
    let t8166 = t1394 * t8165;
    let t8168 = t1982 * t2243;
    let t8169 = t303 * t8168;
    (t8154, t8155, t8158, t8159, t8164, t8165, t8166, t8168, t8169)
}
