//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 906/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk906<F: Float>(t14628: F, t14645: F, t14659: F, t14672: F, t14688: F, t14701: F, t14715: F, t15079: F, t1610: F, t4528: F, t1607: F, t4534: F, t1609: F, t551: F, t1620: F, t4536: F) -> (F, F, F, F, F) {
    let t15082 = t14628 + t14645 + t14659 + t14672 + t14688 + t14701 + t14715 + t15079;
    let t15084 = t4528 * t1610;
    let t15087 = t1607 * t4534;
    let t15092 = t1609 * t1609;
    let t15093 = 1.0 / t15092;
    let t15094 = t551 * t15093;
    let t15095 = t4536 * t1620;
    (t15082, t15084, t15087, t15094, t15095)
}
