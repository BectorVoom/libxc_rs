//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1014/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1014<F: Float>(t155: F, t174: F, t4508: F, t4511: F, t1268: F, t4537: F, t1216: F, t4605: F, t4623: F, t470: F, t1399: F, t4801: F) -> (F, F, F, F) {
    let t18432 = F::new(0.68733717152873822009e1) * t174 * t155 * t4508 * t4511;
    let t18435 = F::new(0.71233333333333333333e-1) * t174 * t1268 * t4537;
    let t18439 = F::new(0.62336721237753107879e3) * t470 * t4605 * t1216 * t4623;
    let t18440 = t1399 * t4801;
    (t18432, t18435, t18439, t18440)
}
