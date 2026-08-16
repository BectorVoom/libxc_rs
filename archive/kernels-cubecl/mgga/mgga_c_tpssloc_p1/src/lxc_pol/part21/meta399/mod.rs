//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1876;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta399<F: Float>(t1557: F, t2793: F, t2842: F, t4434: F, t931: F, t10740: F, t10765: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14419: F, t2861: F, t311: F, t4416: F, t4438: F, t1569: F, t2880: F, t2862: F, t4437: F, t2888: F, t4433: F, t10813: F, t1568: F, t4472: F, t950: F, t1581: F, t2924: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14422, t14424, t14425, t14428) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1876::<F>(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
        let (t14429, t14432, t14436, t14439, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1877::<F>(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
    (t14422, t14424, t14425, t14428, t14429, t14432, t14436, t14439, t14443, t14450, t14453)
}
