//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1033/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1033<F: Float>(t10881: F, t17277: F, t17280: F, t17295: F, t1773: F, t23791: F, t23798: F, t23802: F, t23805: F, t23808: F, t23811: F, t23814: F, t4989: F, t8811: F, t1785: F, t2464: F) -> (F, F) {
    let t23816 = 0.17990788716177317213e-1 * t1773 * t23791 + 0.23987718288236422951e-1 * t4989 * t8811 - 0.32383419689119170984e0 * t1773 * t23798 + 0.59969295720591057377e-2 * t23802 + 0.79959060960788076503e-2 * t23805 - 0.11993859144118211475e-1 * t23808 + 0.23987718288236422951e-1 * t17277 - t17280 - t17295 - 0.35981577432354634427e-1 * t23811 - 0.3997953048039403825e-2 * t10881 + 0.17590993411373376831e0 * t23814;
    let t23819 = t2464 * t1785;
    (t23816, t23819)
}
