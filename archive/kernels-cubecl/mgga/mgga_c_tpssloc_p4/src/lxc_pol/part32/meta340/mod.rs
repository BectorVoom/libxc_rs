//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1377;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta340<F: Float>(t225: F, t4143: F, t4145: F, t1496: F, t9541: F, t2427: F, t4101: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F, t1471: F, t32: F, t4095: F, t67: F, t758: F, t118: F, t1474: F, t2375: F, t4094: F, t706: F, t4162: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13053, t13065, t13087, t13105, t13107, t13109, t13113) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1377::<F>(t225, t4143, t4145, t1496, t9541, t2427, t4101, t2528, t4199, t2663, t4211, t2535);
        let (t13115, t13121, t13124, t13133, t13176) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1378::<F>(t1471, t32, t4095, t67, t758, t118, t1474, t2375, t4094, t706, t4162, t68);
    (t13053, t13065, t13087, t13105, t13107, t13109, t13113, t13115, t13121, t13124, t13133, t13176)
}
