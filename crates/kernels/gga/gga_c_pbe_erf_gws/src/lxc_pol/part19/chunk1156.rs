//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1156/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1156<F: Float>(t15255: F, t51382: F, t3799: F, t4033: F, t3867: F, t11573: F, t14015: F, t11435: F, t51306: F, t14064: F, t3788: F, t15240: F, t8848: F, t3123: F, t54071: F, t11773: F, t14069: F) -> (F, F, F, F, F, F, F, F, F) {
    let t57142 = t51382 * t15255;
    let t57144 = t4033 * t3799;
    let t57146 = t4033 * t3867;
    let t57151 = t14015 * t11573;
    let t57154 = t51306 * t11435;
    let t57156 = t3788 * t14064;
    let t57158 = t8848 * t15240;
    let t57160 = t3123 * t54071;
    let t57162 = t11773 * t14069;
    (t57142, t57144, t57146, t57151, t57154, t57156, t57158, t57160, t57162)
}
