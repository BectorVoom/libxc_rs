//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk492;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk493;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk494;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk495;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk496;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk497;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk498;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk499;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta68(t248: f64, t557: f64, t836: f64, t555: f64, t236: f64, t552: f64, t240: f64, t1336: f64, t531: f64, t556: f64, t241: f64, t67: f64, t1307: f64, t820: f64, t1315: f64, t1327: f64, t1329: f64, t1333: f64, t1341: f64, t1354: f64, t559: f64, t539: f64, t225: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1358 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk492(t248, t557, t836);
        let (t1360, t1361) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk493(t1358, t555, t236, t552);
        let t1362 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk494(t1361, t240);
        let t1363 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk495(t1336, t1362);
        let t1365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk496(t531, t556);
        let t1367 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk497(t1365, t241, t67);
        let t1369 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk498(t1307, t1367, t820);
        let t1372 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk499(t1315, t1327, t1329, t1333, t1341, t1354, t1360, t1363, t1369, t559);
        let (t1373, t1375) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk500(t1372, t539, t225, t563);
    (t1358, t1360, t1361, t1362, t1363, t1365, t1367, t1369, t1372, t1373, t1375)
}
