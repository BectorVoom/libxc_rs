//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2030;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta627<F: Float>(t831: F, t87261: F, t4191: F, t81749: F, t4240: F, t23069: F, t4159: F, t23062: F, t25106: F, t13176: F, t6613: F, t23133: F, t4257: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F, t6620: F, t25097: F, t81782: F, t81783: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87263, t87271, t87273, t87292, t87293, t87295, t87300) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2030::<F>(t831, t87261, t4191, t81749, t4240, t23069, t4159, t23062, t25106, t13176, t6613, t23133, t4257);
        let (t87301, t87304, t87306, t87308, t87321, t87328) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2031::<F>(t87300, t1496, t81942, t7497, t81933, t25098, t81835, t13176, t6620, t25097, t81782, t81783);
    (t87263, t87271, t87273, t87292, t87293, t87295, t87301, t87304, t87306, t87308, t87321, t87328)
}
