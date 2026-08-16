//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1925;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1926;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1927;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta563(t5612: f64, t815: f64, t6605: f64, t1898: f64, t5575: f64, t249: f64, t5628: f64, t6621: f64, t5619: f64, t6614: f64, t23048: f64, t5587: f64, t1512: f64, t25146: f64, t5614: f64, t5617: f64, t2628: f64, t5585: f64, t23096: f64, t23106: f64, t23108: f64, t25065: f64, t26619: f64, t26621: f64, t23146: f64, t5593: f64, t1894: f64, t236: f64, t5544: f64, t6591: f64, t23056: f64, t5568: f64, t5527: f64, t23078: f64, t1484: f64, t1509: f64, t232: f64, t23097: f64, t1516: f64, t25068: f64, t5624: f64, t5572: f64, t6581: f64, t23141: f64, t23144: f64, t25109: f64, t25126: f64, t25133: f64, t26644: f64, t26646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28356, t28357, t28359, t28360, t28362, t28364, t28366) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1925(t5612, t815, t6605, t1898, t5575, t249, t5628, t6621, t5619, t6614, t23048, t5587);
        let (t28372, t28375, t28378) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1926(t1512, t25146, t5614, t6614, t5617, t815, t6605, t2628, t5585, t23096, t23106, t23108, t25065, t26619, t26621, t28357, t28360, t28362, t28364, t28366);
        let (t28380, t28383, t28384, t28386, t28389, t28390, t28395) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1927(t23146, t5593, t1894, t236, t5544, t6591, t23056, t5568, t5527, t23078, t1484, t1509, t232);
        let (t28396, t28405) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1928(t28395, t815, t23097, t1516, t25068, t5624, t6621, t5572, t6581, t23141, t23144, t25109, t25126, t25133, t26644, t26646, t28380, t28384, t28386, t28390);
    (t28356, t28359, t28372, t28375, t28378, t28383, t28389, t28395, t28396, t28405)
}
