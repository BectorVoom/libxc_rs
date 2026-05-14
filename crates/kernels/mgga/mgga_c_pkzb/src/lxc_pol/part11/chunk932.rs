//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 932/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk932<F: Float>(t10767: F, t154: F, t742: F, t10932: F, t5656: F, t287: F, t3542: F, t1137: F, t5693: F, t3645: F, t3679: F, t2105: F, t1123: F, t9562: F, t302: F, t3685: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11011 = t154 * t742 * t10767;
    let t11015 = t154 * t5656 * t10932;
    let t11019 = t287 * t3542;
    let t11020 = t1137 * t11019;
    let t11021 = t5693 * t11020;
    let t11024 = t3679 * t3645;
    let t11025 = t2105 * t11024;
    let t11028 = t1123 * t287;
    let t11029 = t9562 * t11028;
    let t11030 = t302 * t11029;
    let t11033 = t3685 * t3645;
    (t11011, t11015, t11019, t11020, t11021, t11024, t11025, t11028, t11029, t11030, t11033)
}
