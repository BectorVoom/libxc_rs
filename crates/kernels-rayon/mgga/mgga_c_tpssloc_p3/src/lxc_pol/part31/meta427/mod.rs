//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1552;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta427(t1329: f64, t22797: f64, t2230: f64, t6924: f64, t213: f64, t6928: f64, t10: f64, t2229: f64, t60: f64, t1995: f64, t116: f64, t117: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22799, t22803, t22804, t22805, t22811, t22813, t22814, t22815) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1552(t1329, t22797, t2230, t6924, t213, t6928, t10, t2229, t60, t1995, t116, t117);
        let t22816 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1553(t22815, t67);
    (t22799, t22803, t22804, t22805, t22811, t22813, t22814, t22816)
}
