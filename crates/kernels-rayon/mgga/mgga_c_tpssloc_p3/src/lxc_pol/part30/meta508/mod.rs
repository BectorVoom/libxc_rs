//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1827;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta508(t360: f64, t4649: f64, t68: f64, t6744: f64, t344: f64, t7573: f64, t6740: f64, t1622: f64, t23489: f64, t23533: f64, t23537: f64, t23541: f64, t23544: f64, t23554: f64, t23560: f64, t4590: f64, t4596: f64, t4600: f64, t4636: f64, t4652: f64, t6723: f64, t6735: f64, t6742: f64, t6747: f64, t6755: f64, t6765: f64, t7574: f64, t7578: f64, t7583: f64, t25605: f64, t25631: f64, t25672: f64, t383: f64, t4673: f64, t7619: f64, t1598: f64, t984: f64, t23478: f64, t6785: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25678, t25679, t25682, t25683, t25703) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1827(t360, t4649, t68, t6744, t344, t7573, t6740, t1622, t23489, t23533, t23537, t23541, t23544, t23554, t23560, t4590, t4596, t4600, t4636, t4652, t6723, t6735, t6742, t6747, t6755, t6765, t7574, t7578, t7583);
        let (t25705, t25706, t25708, t25712, t25713, t25714, t25717) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1828(t25605, t25631, t25672, t25703, t383, t4673, t7619, t1598, t984, t23478, t6785, t4347);
    (t25678, t25679, t25682, t25683, t25705, t25706, t25708, t25712, t25713, t25714, t25717)
}
