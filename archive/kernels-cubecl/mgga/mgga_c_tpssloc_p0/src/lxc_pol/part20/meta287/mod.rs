//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1487;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1488;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta287<F: Float>(t2887: F, t310: F, t10743: F, t2791: F, t888: F, t2794: F, t2897: F, t942: F, t2929: F, t938: F, t10523: F, t315: F, t10524: F, t2932: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10620: F, t10649: F, t10652: F, t10654: F, t10657: F, t10665: F, t10699: F, t10707: F, t10771: F, t10772: F, t10806: F, t10811: F, t2900: F, t2925: F, t2933: F, t311: F, t924: F, t952: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10813, t10814, t10817, t10819, t10820, t10825, t10828) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1487::<F>(t2887, t310, t10743, t2791, t888, t2794, t2897, t942, t2929, t938, t10523, t315);
        let (t10829, t10832, t10843) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1488::<F>(t10524, t2932, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
        let t10847 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1489::<F>(t10620, t10649, t10652, t10654, t10657, t10665, t10699, t10707, t10771, t10772, t10806, t10811, t10814, t10819, t10820, t10825, t10828, t10829, t10843, t2900, t2925, t2933, t311, t924, t952);
    (t10813, t10814, t10817, t10819, t10820, t10825, t10828, t10829, t10832, t10843, t10847)
}
