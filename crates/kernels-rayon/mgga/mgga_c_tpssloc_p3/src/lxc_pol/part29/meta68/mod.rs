//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk459;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk460;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk461;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk462;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk463;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk464;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk465;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk466;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta68(t1323: f64, t562: f64, t541: f64, t801: f64, t119: f64, t1307: f64, t210: f64, t225: f64, t554: f64, t544: f64, t68: f64, t551: f64, t236: f64, t240: f64, t241: f64, t557: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1324, t1327, t1329) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk459(t1323, t562, t541, t801, t119, t1307, t210);
        let t1332 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk460(t1323, t225);
        let (t1333, t1336) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk461(t1332, t554, t544, t68);
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk462(t551);
        let t1339 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk463(t1338, t236);
        let t1340 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk464(t1339, t240);
        let t1341 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk465(t1336, t1340);
        let t1343 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk466(t241, t557, t67);
    (t1324, t1327, t1329, t1332, t1333, t1336, t1337, t1338, t1339, t1340, t1341, t1343)
}
