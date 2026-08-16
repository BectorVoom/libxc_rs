//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1618;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta369(t17161: f64, t2979: f64, t10214: f64, t17152: f64, t1040: f64, t5904: f64, t248: f64, t3101: f64, t5867: f64, t1020: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t1046: f64, t13750: f64, t13758: f64, t13767: f64, t13946: f64, t17593: f64, t17596: f64, t973: f64) -> (f64, f64, f64, f64) {
        let (t17599, t17602, t17607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1618(t17161, t2979, t10214, t17152, t1040, t5904);
        let (t17611, t17612, t17614) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1619(t248, t3101, t5867, t1020, t10372, t10377, t10381, t10385, t1046, t13750, t13758, t13767, t13946, t17593, t17596, t17599, t17602, t17607, t973);
    (t17607, t17611, t17612, t17614)
}
