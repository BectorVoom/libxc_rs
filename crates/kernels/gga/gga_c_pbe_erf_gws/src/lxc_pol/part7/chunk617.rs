//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 617/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk617<F: Float>(t4791: F, t1422: F, t75: F, t472: F, t1218: F, t1399: F, t1327: F, t414: F, t1319: F, t455: F, t4623: F, t470: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4792 = F::new(0.51947267698127589899e2) * t4791;
    let t4793 = t1422 * t75;
    let t4794 = t4793 * t472;
    let t4795 = F::new(0.17544670192365612213e1) * t4794;
    let t4796 = t1399 * t1218;
    let t4797 = F::new(0.35089340384731224426e1) * t4796;
    let t4798 = t414 * t1327;
    let t4799 = F::new(12.0) * t4798;
    let t4800 = t1319 * t455;
    let t4801 = t4800 * t4623;
    let t4802 = t470 * t4801;
    (t4792, t4793, t4794, t4795, t4796, t4797, t4798, t4799, t4800, t4801, t4802)
}
