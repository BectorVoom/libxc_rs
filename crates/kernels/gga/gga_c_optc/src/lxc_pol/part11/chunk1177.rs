//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1177/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1177<F: Float>(t39545: F, t39560: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t57037: F, t57041: F, t57044: F, t57048: F, t39565: F, t49404: F, t49406: F, t57057: F, t57060: F, t57063: F, t57066: F, t57069: F, t57071: F, t57073: F, t57100: F, t57102: F, t57104: F, t57106: F) -> (F, F) {
    let t57432 = 0.76514814814814814814e0 * t49395 + 0.250068e1 * t57012 + 0.62517e0 * t57016 - 0.104195e0 * t57020 - 0.123954e2 * t57024 - 0.103295e1 * t57027 - 0.125034e1 * t57030 + 0.55570666666666666666e0 * t57034 + 0.68863333333333333334e1 * t57037 - 0.15302962962962962963e1 * t57041 - 0.10805407407407407407e0 * t57044 - 0.104195e0 * t57048 - 0.23154444444444444445e0 * t39545 - 0.69463333333333333334e0 * t39560;
    let t57447 = 0.13892666666666666667e1 * t39565 + 0.166712e1 * t49404 - 0.55570666666666666668e0 * t49406 - 0.516475e0 * t57057 + 0.123954e2 * t57060 + 0.309885e1 * t57063 + 0.94674375e0 * t57066 + 0.2366859375e0 * t57069 - 0.52945875e1 * t57071 - 0.6618234375e1 * t57073 + 0.6311625e0 * t57100 + 0.158837625e2 * t57102 - 0.705945e1 * t57104 + 0.3529725e1 * t57106;
    (t57432, t57447)
}
