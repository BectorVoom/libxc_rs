//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1293/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1293<F: Float>(t1208: F, t2318: F, t22233: F, t18427: F, t18430: F, t18433: F, t18843: F, t22230: F, t22236: F, t22262: F, t352: F, t2239: F, t3030: F) -> (F, F, F, F) {
    let t22699 = t2318 * t1208;
    let t22706 = F::cast_from(0.68493333333333333332e-1_f64) * t22233;
    let t22716 = F::cast_from(0.71233333333333333332e-1_f64) * t22233;
    let t22721 = F::new(0.621814e-1) * (t18843 - F::cast_from(0.16621111111111111111e0_f64) * t18427 + F::cast_from(0.71233333333333333332e-1_f64) * t18430 - F::cast_from(0.17808333333333333333e-1_f64) * t18433 - F::cast_from(0.55403703703703703703e-1_f64) * t22230 + t22716 - F::cast_from(0.53424999999999999999e-1_f64) * t22236 + F::new(0.53425e-1) * t22262) * t352;
    let t22722 = t3030 * t2239;
    (t22699, t22706, t22721, t22722)
}
