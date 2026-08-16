//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 605/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk605(t305: f64, t7817: f64, t648: f64, t7561: f64, t2068: f64, t7638: f64, t118: f64, t326: f64, t4669: f64, t5148: f64, t7538: f64, t7564: f64, t7568: f64, t7574: f64, t7704: f64, t7783: f64, t7786: f64, t7789: f64, t7793: f64, t7796: f64, t7797: f64, t7800: f64, t7803: f64, t7811: f64, t7813: f64, t7816: f64) -> (f64, f64, f64, f64) {
    let t7818 = t305 * t7817;
    let t7819 = 0.14635184302277988245e0_f64 * t7818;
    let t7820 = t648 * t7561;
    let t7821 = 0.33335697577410973224e-1_f64 * t7820;
    let t7826 = t2068 * t7638;
    let t7828 = -0.27274661654245341728e-1_f64 * t7783 + 0.81823984962736025184e-1_f64 * t7786 + 0.20455996240684006296e-1_f64 * t7789 - 0.79828278012425390428e-1_f64 * t118 * t7568 + 0.79828278012425390426e-1_f64 * t7793 + t7796 + 0.2993560425465952141e-1_f64 * t7797 - 0.35922725105591425692e0_f64 * t4669 * t7800 - 0.23948483403727617128e0_f64 * t5148 * t7803 + 0.11974241701863808564e0_f64 * t305 * t7538 - 0.39914139006212695214e-1_f64 * t118 * t7574 + 0.2993560425465952141e-1_f64 * t7811 - 0.44903406381989282115e-1_f64 * t7813 - t7816 + t7819 - t7821 + 0.11974241701863808564e0_f64 * t118 * t7704 - 0.11974241701863808564e0_f64 * t326 * t7564 + 0.54549323308490683457e-1_f64 * t7826;
    (t7819, t7821, t7826, t7828)
}
