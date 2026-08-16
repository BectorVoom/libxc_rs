//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1880;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1881;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta401(t14473: f64, t961: f64, t2948: f64, t4483: f64, t14364: f64, t300: f64, t2907: f64, t4496: f64, t959: f64, t2952: f64, t10623: f64, t1589: f64, t14257: f64, t14262: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t14391: f64, t14394: f64, t14398: f64, t14424: f64, t14472: f64, t14238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14475, t14477, t14479, t14480, t14482, t14484, t14486) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1880(t14473, t961, t2948, t4483, t14364, t300, t2907, t4496, t959, t2952, t10623, t1589);
        let t14487 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1881(t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486);
        let t14488 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1882(t14238, t14487);
    (t14475, t14477, t14479, t14480, t14482, t14484, t14486, t14488)
}
