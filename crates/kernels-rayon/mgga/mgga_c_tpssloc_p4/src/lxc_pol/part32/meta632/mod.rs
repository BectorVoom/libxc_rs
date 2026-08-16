//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta632(t13176: f64, t6620: f64, t25097: f64, t81782: f64, t81783: f64, t1516: f64, t81769: f64, t23133: f64, t4261: f64, t25111: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64, t849: f64, t81763: f64, t23083: f64, t25094: f64, t23046: f64, t4184: f64, t812: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87321, t87329, t87331, t87333, t87336, t87338) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2044(t13176, t6620, t25097, t81782, t81783, t1516, t81769, t23133, t4261, t25111, t25115, t87229);
        let (t87339, t87340, t87342, t87345, t87348, t87363) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2045(t87338, t23132, t4166, t849, t1516, t81763, t23083, t25094, t23046, t4184, t812, t836);
    (t87321, t87329, t87331, t87333, t87336, t87339, t87340, t87342, t87345, t87348, t87363)
}
