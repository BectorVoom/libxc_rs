//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1221;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta408<F: Float>(t15363: F, t4889: F, t11529: F, t1174: F, t6126: F, t44571: F, t6119: F, t3030: F, t6150: F, t3609: F, t3623: F, t15730: F, t5019: F, t3508: F, t6218: F, t11721: F, t6224: F, t11818: F, t1213: F, t248: F, t6219: F, t3036: F, t6163: F, t3500: F, t3503: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t65023, t65112, t65126, t65253, t65254, t65262, t65444) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1221::<F>(t15363, t4889, t11529, t1174, t6126, t44571, t6119, t3030, t6150, t3609, t3623, t15730, t5019);
        let (t65464, t65474, t65528, t65539, t65541) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1222::<F>(t3508, t6218, t11721, t6224, t11818, t1213, t248, t6219, t3036, t6163, t3500, t3503);
    (t65023, t65112, t65126, t65253, t65254, t65262, t65444, t65464, t65474, t65528, t65539, t65541)
}
