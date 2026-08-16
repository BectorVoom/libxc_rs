//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk979;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk980;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta189<F: Float>(t1474: F, t67: F, t758: F, t2431: F, t2532: F, t2653: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2538: F, t2665: F, t225: F, t4210: F, t228: F, t68: F, t1484: F, t845: F, t776: F, t4119: F, t824: F, t1504: F, t1506: F, t230: F, t822: F, t825: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4211, t4213, t4214, t4215, t4216, t4217) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk979::<F>(t1474, t67, t758, t2431, t2532, t2653, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2538, t2665);
        let (t4219, t4225, t4226, t4227, t4230, t4233) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk980::<F>(t225, t4210, t4217, t228, t68, t1484, t845, t776, t4119, t824, t1504, t1506, t230, t822, t825);
        let t4234 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk981::<F>(t232, t4233);
    (t4211, t4213, t4214, t4215, t4216, t4219, t4225, t4226, t4227, t4230, t4233, t4234)
}
