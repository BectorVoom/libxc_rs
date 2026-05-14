//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 768/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk768<F: Float>(t1196: F, t6290: F, t2320: F, t3135: F, t1208: F, t6233: F, t1184: F, t6201: F, t1189: F, t2256: F, t3030: F, t832: F, t1171: F, t2239: F) -> (F, F, F, F, F, F, F) {
    let t8153 = t1196 * t6290;
    let t8170 = t3135 * t2320;
    let t8177 = t1208 * t6233;
    let t8205 = t1184 * t6201;
    let t8211 = t1189 * t2256;
    let t8214 = t3030 * t832;
    let t8219 = t1171 * t2239;
    (t8153, t8170, t8177, t8205, t8211, t8214, t8219)
}
