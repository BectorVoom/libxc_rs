//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta677<F: Float>(t14855: F, t3411: F, t14933: F, t300: F, t1166: F, t3401: F, t1155: F, t3395: F, t1695: F, t11292: F, t1164: F, t3404: F, t4857: F, t11310: F, t15225: F, t51725: F, t51399: F, t51401: F, t51404: F, t51437: F, t51439: F, t51441: F, t51443: F, t51446: F, t51449: F, t51453: F, t51456: F, t51459: F, t51463: F, t51466: F, t51470: F, t51472: F, t11433: F, t14966: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51806, t51809, t51811, t51814, t51818) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558::<F>(t14855, t3411, t14933, t300, t1166, t3401, t1155, t3395, t1695, t11292, t1164, t3404, t4857);
        let (t51822, t51824, t51825) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559::<F>(t11310, t300, t15225, t51811, t51725, t51399, t51401, t51404, t51437, t51439, t51806, t51809, t51814, t51818);
        let (t51826, t51831) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2560::<F>(t51441, t51443, t51446, t51449, t51453, t51456, t51459, t51463, t51466, t51470, t51472, t11433, t1164, t14966);
    (t51806, t51809, t51811, t51814, t51818, t51822, t51824, t51825, t51826, t51831)
}
