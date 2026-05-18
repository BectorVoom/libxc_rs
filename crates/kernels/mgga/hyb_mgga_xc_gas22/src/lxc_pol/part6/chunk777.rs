//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 777/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk777<F: Float>(t2289: F, t4180: F, t848: F, t4193: F, t839: F, t2311: F, t2314: F, t1379: F, t260: F, t3430: F, t4110: F, t4112: F, t4116: F, t4142: F, t4145: F, t4176: F, t4200: F, t856: F) -> (F, F, F, F, F) {
    let t4207 = t2289 * t4180 * t848;
    let t4211 = t839 * t4193 * t848;
    let t4214 = t2311 * t4180;
    let t4215 = t4214 * t2314;
    let t4218 = -t4110 + t4112 - t4116 + t4142 + t4145 + t260 * t4200 + F::new(0.19751673498613801407e-1) * t260 * t4176 - F::new(0.11696447245269292414e1) * t3430 * t1379 + F::new(0.11696447245269292414e1) * t856 * t4207 - F::new(0.5848223622634646207e0) * t856 * t4211 - F::new(0.17315859105681463759e2) * t856 * t4215;
    (t4207, t4211, t4214, t4215, t4218)
}
