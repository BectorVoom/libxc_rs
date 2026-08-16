//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 658/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk658(t27: f64, t3839: f64, t1635: f64, t649: f64, t3826: f64, t1624: f64, t1627: f64, t7603: f64, t8729: f64, t8731: f64, t8733: f64, t8735: f64, t8737: f64, t8739: f64, t8741: f64, t8744: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8746 = t3839 * t27;
    let t8747 = t649 * t1635;
    let t8748 = t8746 * t8747;
    let t8750 = t3826 * t27;
    let t8751 = t649 * t1624;
    let t8752 = t8750 * t8751;
    let t8754 = t649 * t1627;
    let t8755 = t7603 * t8754;
    let t8757 = 0.14967802127329760705e-1_f64 * t8729 - 0.99785347515531738034e-2_f64 * t8731 - 0.99785347515531738034e-2_f64 * t8733 + 0.88507694033737208925e-3_f64 * t8735 - 0.10620923284048465071e-2_f64 * t8737 - 0.39914139006212695213e-1_f64 * t8739 + 0.26609426004141796809e-1_f64 * t8741 - 0.13637330827122670865e-1_f64 * t8744 + 0.22728884711871118108e-1_f64 * t8748 + 0.45360193192290319575e-3_f64 * t8752 - 0.63504270469206447405e-3_f64 * t8755;
    (t8746, t8747, t8750, t8751, t8754, t8757)
}
