//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1876;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta399(t1557: f64, t2793: f64, t2842: f64, t4434: f64, t931: f64, t10740: f64, t10765: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t14391: f64, t14394: f64, t14398: f64, t14419: f64, t2861: f64, t311: f64, t4416: f64, t4438: f64, t1569: f64, t2880: f64, t2862: f64, t4437: f64, t2888: f64, t4433: f64, t10813: f64, t1568: f64, t4472: f64, t950: f64, t1581: f64, t2924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14422, t14424, t14425, t14428) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1876(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
        let (t14429, t14432, t14436, t14439, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1877(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
    (t14422, t14424, t14425, t14428, t14429, t14432, t14436, t14439, t14443, t14450, t14453)
}
