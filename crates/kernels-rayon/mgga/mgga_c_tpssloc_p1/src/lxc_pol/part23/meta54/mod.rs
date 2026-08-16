//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta54 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk337;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk338;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk339;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk340;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta54(t1365: f64, t241: f64, t67: f64, t225: f64, t563: f64, t566: f64, t68: f64, t1338: f64, t562: f64, t570: f64, t3: f64, t576: f64, t112: f64, t582: f64, t586: f64, t589: f64, t593: f64, t596: f64, t600: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1367, t1375) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk337(t1365, t241, t67, t225, t563);
        let (t1376, t1378, t1380) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk338(t566, t68, t1338, t562);
        let t1390 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk339(t570);
        let (t1398, t1401, t1406, t1408) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk340(t3, t576, t112, t582, t586, t589, t593, t596, t600, t4, t581);
        let t1409 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk341(t25, t28, t1408, zeta_threshold);
    (t1367, t1375, t1376, t1378, t1380, t1390, t1398, t1401, t1406, t1408, t1409)
}
