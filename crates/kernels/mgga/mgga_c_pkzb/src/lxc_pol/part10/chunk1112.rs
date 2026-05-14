//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1112/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1112<F: Float>(t3779: F, t6290: F, t870: F, t6088: F, t6090: F, t7955: F, t8233: F, t9782: F, t9797: F, t352: F, t10000: F, t10003: F, t10006: F, t10009: F, t10013: F, t10016: F, t2257: F, t2279: F, t2318: F, t3088: F, t3107: F, t6282: F, t6288: F, t6313: F, t8120: F, t8211: F, t9986: F, t9989: F, t9993: F) -> (F, F, F, F, F) {
    let t10019 = t3779 * t6290;
    let t10020 = t10019 * t870;
    let t10027 = -t6088 + 0.23744444444444444444e-1 * t6090 + 0.47488888888888888888e-1 * t7955 - t8233 - 0.17808333333333333333e-1 * t9782 + 0.53425e-1 * t9797;
    let t10029 = 0.621814e-1 * t10027 * t352;
    let t10030 = 0.17315859105681463759e2 * t2318 * t9986 + 0.34631718211362927518e2 * t2318 * t9989 + 0.10254018858216406658e4 * t6282 * t9993 - 4.0 * t8211 * t3088 + 0.64327917994770140268e2 * t8120 * t3107 + 6.0 * t2279 * t10000 - 4.0 * t2257 * t10003 - 0.19298375398431042081e3 * t6313 * t10006 - 2.0 * t2257 * t10009 + 0.32163958997385070134e2 * t2279 * t10013 + 0.64327917994770140268e2 * t2279 * t10016 + 0.2069040516770936012e4 * t6288 * t10020 + t10029;
    (t10019, t10020, t10027, t10029, t10030)
}
