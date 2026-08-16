//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1843;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta427<F: Float>(t2906: F, t4475: F, t2932: F, t4471: F, t950: F, t1581: F, t1569: F, t2862: F, t10747: F, t10771: F, t10811: F, t10825: F, t10828: F, t14429: F, t14432: F, t14436: F, t14439: F, t14443: F, t14450: F, t14453: F, t2861: F, t2886: F, t2905: F, t2930: F, t4454: F, t4476: F, t14279: F, t14373: F, t14428: F, t300: F, t4446: F, t961: F, t2948: F, t4483: F, t14364: F, t2907: F, t4496: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14456, t14459, t14460, t14463, t14466, t14469) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1843::<F>(t2906, t4475, t2932, t4471, t950, t1581, t1569, t2862, t10747, t10771, t10811, t10825, t10828, t14429, t14432, t14436, t14439, t14443, t14450, t14453, t2861, t2886, t2905, t2930, t4454, t4476);
        let (t14472, t14473, t14475, t14477, t14479, t14480) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1844::<F>(t14279, t14373, t14428, t14469, t300, t4446, t961, t2948, t4483, t14364, t2907, t4496);
    (t14456, t14459, t14460, t14463, t14466, t14472, t14473, t14475, t14477, t14479, t14480)
}
