//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2164;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2165;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta515(t1022: f64, t10482: f64, t17670: f64, t4582: f64, t1539: f64, t4650: f64, t3071: f64, t5867: f64, t884: f64, t10390: f64, t1041: f64, t10480: f64, t10904: f64, t13995: f64, t14000: f64, t14027: f64, t17643: f64, t17649: f64, t17656: f64, t17660: f64, t17662: f64, t17668: f64, t3070: f64, t4575: f64, t5875: f64, t5909: f64, t5392: f64, t607: f64, t14172: f64, t1409: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17671, t17672, t17673, t17676, t17677, t17680, t17681, t17684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2164(t1022, t10482, t17670, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2165(t5392, t607);
        let (t17687, t17688, t17691) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2166(t14172, t17686, t4582, t1409, t3966);
    (t17671, t17672, t17673, t17676, t17677, t17680, t17681, t17684, t17686, t17687, t17688, t17691)
}
