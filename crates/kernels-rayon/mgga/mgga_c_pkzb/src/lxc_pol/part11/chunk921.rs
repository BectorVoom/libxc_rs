//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 921/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk921(t10027: f64, t352: f64, t10000: f64, t10003: f64, t10006: f64, t10009: f64, t10013: f64, t10016: f64, t10020: f64, t2257: f64, t2279: f64, t2318: f64, t3088: f64, t3107: f64, t6282: f64, t6288: f64, t6313: f64, t8120: f64, t8211: f64, t9986: f64, t9989: f64, t9993: f64) -> (f64, f64) {
    let t10029 = 0.621814e-1_f64 * t10027 * t352;
    let t10030 = 0.17315859105681463759e2_f64 * t2318 * t9986 + 0.34631718211362927518e2_f64 * t2318 * t9989 + 0.10254018858216406658e4_f64 * t6282 * t9993 - 4.0_f64 * t8211 * t3088 + 0.64327917994770140268e2_f64 * t8120 * t3107 + 6.0_f64 * t2279 * t10000 - 4.0_f64 * t2257 * t10003 - 0.19298375398431042081e3_f64 * t6313 * t10006 - 2.0_f64 * t2257 * t10009 + 0.32163958997385070134e2_f64 * t2279 * t10013 + 0.64327917994770140268e2_f64 * t2279 * t10016 + 0.2069040516770936012e4_f64 * t6288 * t10020 + t10029;
    (t10029, t10030)
}
