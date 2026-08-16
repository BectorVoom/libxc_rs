//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1470;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta304<F: Float>(t10704: F, t1556: F, t13566: F, t13602: F, t10813: F, t1568: F, t2932: F, t4471: F, t300: F, t4446: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F, t3032: F, t3129: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14395, t14409, t14410, t14442, t14459, t14473, t14495) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1470::<F>(t10704, t1556, t13566, t13602, t10813, t1568, t2932, t4471, t300, t4446, t3053, t4644);
        let (t14501, t14503, t14506, t14507, t14508) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1471::<F>(t10422, t4578, t3070, t1603, t3030, t3032, t3129);
    (t14395, t14409, t14410, t14442, t14459, t14473, t14495, t14501, t14503, t14506, t14507, t14508)
}
