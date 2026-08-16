//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2247;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta682<F: Float>(t17183: F, t2970: F, t973: F, t10231: F, t17178: F, t10390: F, t18041: F, t10422: F, t18024: F, t3070: F, t13969: F, t17733: F, t3130: F, t17152: F, t42972: F, t10876: F, t17983: F, t13995: F, t14501: F, t18020: F, t10883: F, t17979: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62663, t62666, t62682, t62687, t62704) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2247::<F>(t17183, t2970, t973, t10231, t17178, t10390, t18041, t10422, t18024, t3070, t13969, t17733, t3130);
        let (t62766, t62778, t62780, t62811, t62816) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2248::<F>(t17152, t42972, t973, t10876, t13969, t17983, t13995, t14501, t10422, t18020, t3070, t10883, t17979);
    (t62663, t62666, t62682, t62687, t62704, t62766, t62778, t62780, t62811, t62816)
}
