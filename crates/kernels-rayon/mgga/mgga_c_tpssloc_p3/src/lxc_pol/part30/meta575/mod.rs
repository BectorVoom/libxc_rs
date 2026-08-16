//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1948;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1949;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta575(t28523: f64, t28718: f64, t1915: f64, t5527: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t23295: f64, t2522: f64, t25358: f64, t28248: f64, t28447: f64, t4314: f64, t5544: f64, t5660: f64, t5664: f64, t6670: f64, t7541: f64, t870: f64, t265: f64, t394: f64, t1070: f64, t1637: f64, t23742: f64, t25840: f64, t336: f64, t4700: f64, t5946: f64, t5950: f64, t6822: f64, t25: f64, t1409: f64, t1965: f64, t28469: f64, t40: f64, t5398: f64, t7643: f64, t28: f64, t23788: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28719, t28732, t28755) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1948(t28523, t28718, t1915, t5527, t1484, t1530, t1877, t193, t202, t23295, t2522, t25358, t28248, t28447, t4314, t5544, t5660, t5664, t6670, t7541, t870);
        let t28756 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1949(t265, t394, t1070, t1637, t193, t23742, t25840, t28719, t28755, t336, t4700, t5946, t5950, t6822);
        let (t28763, t28764, t28765, t28771) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1950(t25, t1409, t1965, t28469, t28756, t40, t5398, t7643, t28, t5527, t1915, t23788, t28248, dens_threshold, rho0, zeta_threshold);
    (t28719, t28732, t28755, t28756, t28763, t28764, t28765, t28771)
}
