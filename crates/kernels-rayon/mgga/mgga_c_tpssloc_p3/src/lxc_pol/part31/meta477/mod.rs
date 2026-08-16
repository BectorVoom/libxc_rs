//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1638;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta477(t26309: f64, t5252: f64, t22833: f64, t5293: f64, t5303: f64, t1351: f64, t16311: f64, t3788: f64, t6936: f64, t16306: f64, t550: f64, t1339: f64, t1887: f64, t22839: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1638(t26309, t5252, t22833, t5293, t5303, t1351, t16311, t3788, t6936, t16306, t550, t1339);
        let (t26324, t26331) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1639(t26323, t6936, t1887, t22839);
    (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323, t26324, t26331)
}
