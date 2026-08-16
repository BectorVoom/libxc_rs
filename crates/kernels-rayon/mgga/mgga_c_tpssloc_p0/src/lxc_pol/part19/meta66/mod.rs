//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk413;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk414;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk415;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk416;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta66(t248: f64, t557: f64, t836: f64, t555: f64, t236: f64, t552: f64, t240: f64, t1336: f64, t531: f64, t556: f64, t241: f64, t67: f64, t1307: f64, t820: f64, t1315: f64, t1327: f64, t1329: f64, t1333: f64, t1341: f64, t1354: f64, t559: f64, t539: f64, t225: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1358, t1360, t1361, t1362, t1363) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk413(t248, t557, t836, t555, t236, t552, t240, t1336);
        let t1365 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk414(t531, t556);
        let (t1367, t1369) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk415(t1365, t241, t67, t1307, t820);
        let t1372 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk416(t1315, t1327, t1329, t1333, t1341, t1354, t1360, t1363, t1369, t559);
        let (t1373, t1375) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk417(t1372, t539, t225, t563);
    (t1358, t1361, t1362, t1363, t1365, t1367, t1369, t1372, t1373, t1375)
}
