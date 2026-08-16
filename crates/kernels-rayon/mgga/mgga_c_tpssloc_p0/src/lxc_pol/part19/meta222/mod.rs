//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk923;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk924;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta222(t2897: f64, t942: f64, t2929: f64, t938: f64, t10523: f64, t315: f64, t10524: f64, t2932: f64, t10544: f64, t10530: f64, t10538: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10620: f64, t10649: f64, t10652: f64, t10654: f64, t10657: f64, t10665: f64, t10699: f64, t10707: f64, t10771: f64, t10772: f64, t10806: f64, t10811: f64, t10814: f64, t10819: f64, t2900: f64, t2925: f64, t2933: f64, t311: f64, t924: f64, t952: f64, t10768: f64, t300: f64, t2940: f64, t2944: f64, t2924: f64, t4497: f64, t959: f64, t10711: f64, t10715: f64, t10729: f64, t10733: f64, t10739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10820, t10825, t10828, t10829, t10843) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk923(t2897, t942, t2929, t938, t10523, t315, t10524, t2932, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
        let t10847 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk924(t10620, t10649, t10652, t10654, t10657, t10665, t10699, t10707, t10771, t10772, t10806, t10811, t10814, t10819, t10820, t10825, t10828, t10829, t10843, t2900, t2925, t2933, t311, t924, t952);
        let (t10849, t10851, t10853, t10855, t10856) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk925(t10768, t10847, t300, t2940, t2944, t2924, t2929, t4497, t959, t10665, t10699, t10707, t10711, t10715, t10729, t10733, t10739, t10819);
    (t10820, t10825, t10828, t10829, t10843, t10849, t10851, t10853, t10855, t10856)
}
