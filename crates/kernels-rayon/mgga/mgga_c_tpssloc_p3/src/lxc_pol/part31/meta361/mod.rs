//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1281;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta361(t5151: f64, t67: f64, t758: f64, t12365: f64, t1827: f64, t12300: f64, t12418: f64, t820: f64, t1351: f64, t1799: f64, t12289: f64, t242: f64, t1336: f64, t12283: f64, t5259: f64, t5293: f64, t120: f64, t5286: f64, t5303: f64, t1340: f64, t16060: f64, t3798: f64, t5234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16171, t16211, t16214, t16224, t16225, t16232) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1281(t5151, t67, t758, t12365, t1827, t12300, t12418, t820, t1351, t1799, t12289, t242);
        let (t16233, t16239, t16241, t16242, t16269, t16278, t16288) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1282(t1336, t16232, t12283, t5259, t5293, t120, t5286, t5303, t1340, t16060, t3798, t5234);
    (t16171, t16211, t16214, t16224, t16225, t16233, t16239, t16241, t16242, t16269, t16278, t16288)
}
