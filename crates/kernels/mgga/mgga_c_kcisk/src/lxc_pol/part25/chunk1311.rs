//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1311/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1311<F: Float>(t116474: F, t9649: F, t16627: F, t1799: F, t33017: F, t116350: F, t112216: F, t112236: F, t112244: F, t112530: F, t112534: F, t116397: F, t116469: F, t32952: F, t32959: F, t33023: F, t34125: F, t9922: F, t9936: F) -> (F, F) {
    let t116856 = 0.26805555555555555556e-2 * t9649 * t116474;
    let t116859 = t1799 * t33017 * t16627;
    let t116866 = t9649 * t116350;
    let t116878 = 0.8041666666666666667e-2 * t9649 * t116469 - t116856 + 0.23148148148148148148e-2 * t112530 - 0.16581944444444444444e-2 * t116859 - 0.26805555555555555556e-2 * t112534 + 0.40208333333333333335e-2 * t112236 * t9922 + 0.55555555555555555558e-1 * t34125 * t33023 - 0.89351851851851851853e-3 * t116866 - 0.8041666666666666667e-2 * t9649 * t116397 + 0.92592592592592592597e-2 * t34125 * t32959 + 0.12345679012345679013e-1 * t34125 * t32952 - 0.34722222222222222223e-2 * t112216 * t9936 - 0.69444444444444444446e-2 * t112244 * t9936;
    (t116859, t116878)
}
