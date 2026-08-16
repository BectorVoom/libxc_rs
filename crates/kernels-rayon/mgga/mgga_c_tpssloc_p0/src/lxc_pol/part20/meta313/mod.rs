//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1571;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1572;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1573;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta313(t1138: f64, t3351: f64, t1136: f64, t3359: f64, t11135: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t11165: f64, t11170: f64, t11174: f64, t423: f64, t11177: f64, t11365: f64, t11366: f64, t11400: f64, t11405: f64, t11409: f64, t11410: f64, t11415: f64, t11420: f64, t11421: f64, t11426: f64, t11429: f64, t11430: f64, t11434: f64, t1148: f64, t3327: f64, t3332: f64, t3352: f64, t3357: f64, t3360: f64, t3376: f64, t3401: f64, t436: f64, t11364: f64, t300: f64, t11128: f64, t11133: f64, t11179: f64, t11182: f64, t11184: f64, t11187: f64, t11194: f64, t11272: f64, t11280: f64, t11288: f64, t11290: f64, t11296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11437, t11441, t11444, t11455) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1571(t1138, t3351, t1136, t3359, t11135, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174);
        let (t11459, t11470, t11472) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1572(t11135, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174, t423);
        let t11473 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1573(t11177, t11365, t11366, t1138, t11400, t11405, t11409, t11410, t11415, t11420, t11421, t11426, t11429, t11430, t11434, t11437, t11441, t11455, t11472, t1148, t3327, t3332, t3352, t3357, t3360, t3376, t3401, t436);
        let (t11475, t11476) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1574(t11364, t11473, t300, t11128, t11133, t11179, t11182, t11184, t11187, t11194, t11272, t11280, t11288, t11290, t11296);
    (t11437, t11441, t11444, t11455, t11459, t11470, t11472, t11475, t11476)
}
