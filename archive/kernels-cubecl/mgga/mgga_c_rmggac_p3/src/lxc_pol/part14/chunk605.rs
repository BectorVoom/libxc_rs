//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 605/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk605<F: Float>(t305: F, t7817: F, t648: F, t7561: F, t2068: F, t7638: F, t118: F, t326: F, t4669: F, t5148: F, t7538: F, t7564: F, t7568: F, t7574: F, t7704: F, t7783: F, t7786: F, t7789: F, t7793: F, t7796: F, t7797: F, t7800: F, t7803: F, t7811: F, t7813: F, t7816: F) -> (F, F, F, F) {
    let t7818 = t305 * t7817;
    let t7819 = F::cast_from(0.14635184302277988245e0_f64) * t7818;
    let t7820 = t648 * t7561;
    let t7821 = F::cast_from(0.33335697577410973224e-1_f64) * t7820;
    let t7826 = t2068 * t7638;
    let t7828 = -F::cast_from(0.27274661654245341728e-1_f64) * t7783 + F::cast_from(0.81823984962736025184e-1_f64) * t7786 + F::cast_from(0.20455996240684006296e-1_f64) * t7789 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t7568 + F::cast_from(0.79828278012425390426e-1_f64) * t7793 + t7796 + F::cast_from(0.2993560425465952141e-1_f64) * t7797 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t7800 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t7803 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t7538 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t7574 + F::cast_from(0.2993560425465952141e-1_f64) * t7811 - F::cast_from(0.44903406381989282115e-1_f64) * t7813 - t7816 + t7819 - t7821 + F::cast_from(0.11974241701863808564e0_f64) * t118 * t7704 - F::cast_from(0.11974241701863808564e0_f64) * t326 * t7564 + F::cast_from(0.54549323308490683457e-1_f64) * t7826;
    (t7819, t7821, t7826, t7828)
}
