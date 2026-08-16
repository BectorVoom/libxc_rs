//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta335(t1339: f64, t2690: f64, t1336: f64, t1354: f64, t1307: f64, t3792: f64, t3788: f64, t835: f64, t1995: f64, t67: f64, t246: f64, t3777: f64, t3802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12365, t12366, t12369, t12385, t12418, t12419, t12429) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1368(t1339, t2690, t1336, t1354, t1307, t3792, t3788, t835, t1995, t67, t246, t3777, t3802);
    (t12365, t12366, t12369, t12385, t12418, t12419, t12429)
}
