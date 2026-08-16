//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1520;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1521;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1522;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta399(t1041: f64, t17659: f64, t4630: f64, t4641: f64, t248: f64, t3101: f64, t5873: f64, t3130: f64, t376: f64, t5872: f64, t1022: f64, t10482: f64, t4582: f64, t1539: f64, t4650: f64, t3071: f64, t5867: f64, t884: f64, t10390: f64, t10480: f64, t10904: f64, t13995: f64, t14000: f64, t14027: f64, t17643: f64, t17649: f64, t17656: f64, t3070: f64, t4575: f64, t5875: f64, t5909: f64, t5392: f64, t607: f64, t14172: f64, t1409: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17660, t17662, t17667, t17668, t17670, t17671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1520(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5872, t1022, t10482);
        let (t17673, t17677, t17681, t17684) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1521(t17670, t17671, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1522(t5392, t607);
        let (t17688, t17691) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1523(t14172, t17686, t4582, t1409, t3966);
    (t17667, t17670, t17671, t17673, t17677, t17681, t17684, t17686, t17688, t17691)
}
