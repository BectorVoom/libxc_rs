//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 968/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk968<F: Float>(t40803: F, t40831: F, t118: F, t305: F, t326: F, t40806: F, t40809: F, t40814: F, t40824: F, t40827: F, t40834: F, t43080: F, t43644: F, t43749: F, t43971: F, t40842: F, t40844: F, t40846: F, t40850: F, t40852: F, t40854: F, t40856: F, t40858: F, t40860: F, t40862: F, t40866: F, t40868: F, t40870: F) -> (F, F) {
    let t44029 = 0.3193131120497015617e0 * t40803;
    let t44035 = 0.3193131120497015617e0 * t40831;
    let t44043 = -0.79828278012425390428e-1 * t118 * t43971 - t44029 - 0.47896966807455234256e0 * t40806 - 0.17961362552795712846e0 * t40809 - 0.2993560425465952141e-1 * t40814 - 0.35922725105591425692e0 * t40824 - 0.11974241701863808564e0 * t40827 + t44035 - 0.35922725105591425692e0 * t40834 + 0.59871208509319042821e-1 * t305 * t43080 - 0.11974241701863808564e0 * t326 * t43644 - 0.59871208509319042821e-1 * t326 * t43749;
    let t44057 = 0.5987120850931904282e-1 * t40842 + 0.16364796992547205038e0 * t40844 + 0.8182398496273602519e-1 * t40846 - 0.2727466165424534173e0 * t40850 - 0.13637330827122670865e0 * t40852 - 0.5454932330849068346e-1 * t40854 - 0.2727466165424534173e-1 * t40856 + 0.40911992481368012596e-1 * t40858 + 0.20455996240684006298e-1 * t40860 - 0.5454932330849068346e-1 * t40862 - 0.2727466165424534173e-1 * t40866 - 0.2727466165424534173e-1 * t40868 - 0.13637330827122670865e-1 * t40870;
    (t44043, t44057)
}
