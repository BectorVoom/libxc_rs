//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2544/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544<F: Float>(t10403: F, t10422: F, t14214: F, t3030: F, t4552: F, t3032: F, t3129: F, t13998: F, t2960: F, t42875: F, t4338: F, t973: F) -> (F, F, F, F, F, F) {
    let t49629 = t10403 * t10422 * t14214;
    let t49649 = t4552 * t3030;
    let t49650 = t49649 * t3032;
    let t49651 = t49650 * t3129;
    let t49658 = t2960 * t13998;
    let t49661 = t973 * t42875 * t4338;
    (t49629, t49649, t49650, t49651, t49658, t49661)
}
