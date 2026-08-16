//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta478(t1164: f64, t43689: f64, t43692: f64, t78287: f64, t18622: f64, t64451: f64, t21833: f64, t4869: f64, t5989: f64, t64257: f64, t11292: f64, t1156: f64, t22237: f64, t78242: f64, t78247: f64, t78250: f64, t78254: f64, t78281: f64, t78283: f64, t78286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t78291, t78294, t78296, t78298, t78302) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432(t1164, t43689, t43692, t78287, t18622, t64451, t21833, t4869, t5989, t64257, t11292, t1156);
        let (t78304, t78305) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433(t22237, t4869, t78242, t78247, t78250, t78254, t78281, t78283, t78286, t78291, t78294, t78296, t78298, t78302);
    (t78291, t78294, t78296, t78298, t78302, t78304, t78305)
}
