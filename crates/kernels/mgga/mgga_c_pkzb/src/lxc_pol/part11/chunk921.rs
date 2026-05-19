//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 921/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk921<F: Float>(t10027: F, t352: F, t10000: F, t10003: F, t10006: F, t10009: F, t10013: F, t10016: F, t10020: F, t2257: F, t2279: F, t2318: F, t3088: F, t3107: F, t6282: F, t6288: F, t6313: F, t8120: F, t8211: F, t9986: F, t9989: F, t9993: F) -> (F, F) {
    let t10029 = F::new(0.621814e-1) * t10027 * t352;
    let t10030 = F::cast_from(0.17315859105681463759e2_f64) * t2318 * t9986 + F::cast_from(0.34631718211362927518e2_f64) * t2318 * t9989 + F::cast_from(0.10254018858216406658e4_f64) * t6282 * t9993 - F::new(4.0) * t8211 * t3088 + F::cast_from(0.64327917994770140268e2_f64) * t8120 * t3107 + F::new(6.0) * t2279 * t10000 - F::new(4.0) * t2257 * t10003 - F::cast_from(0.19298375398431042081e3_f64) * t6313 * t10006 - F::new(2.0) * t2257 * t10009 + F::cast_from(0.32163958997385070134e2_f64) * t2279 * t10013 + F::cast_from(0.64327917994770140268e2_f64) * t2279 * t10016 + F::cast_from(0.2069040516770936012e4_f64) * t6288 * t10020 + t10029;
    (t10029, t10030)
}
