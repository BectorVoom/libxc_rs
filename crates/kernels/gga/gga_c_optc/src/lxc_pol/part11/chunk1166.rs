//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1166/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1166<F: Float>(t23926: F, t23927: F, t30189: F, t30270: F, t49378: F, t49381: F, t49385: F, t49387: F, t49393: F, t56988: F, t56991: F, t56994: F, t56997: F, t56999: F, t39545: F, t39560: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t57037: F, t57041: F, t57044: F, t57048: F) -> (F, F) {
    let t57148 = -0.298026e1 * t56988 + 0.66228e0 * t56991 + 0.99342e0 * t56994 + 0.98115555555555555556e0 * t30189 + t23926 + t23927 - 0.247573125e0 * t56997 + 0.3300975e0 * t56999 + 0.98115555555555555555e-1 * t49378 + 0.22076e0 * t49381 + 0.12524296296296296297e1 * t30270 - 0.16102666666666666667e1 * t49385 + 0.24154e1 * t49387 + 0.40256666666666666668e0 * t49393;
    let t57164 = 0.44729629629629629629e0 * t49395 + 0.198684e1 * t57012 + 0.49671e0 * t57016 - 0.82785e-1 * t57020 - 0.72462e1 * t57024 - 0.60384999999999999999e0 * t57027 - 0.99342e0 * t57030 + 0.44152e0 * t57034 + 0.40256666666666666666e1 * t57037 - 0.89459259259259259259e0 * t57041 - 0.8585111111111111111e-1 * t57044 - 0.82785e-1 * t57048 - 0.18396666666666666667e0 * t39545 - 0.5519e0 * t39560;
    (t57148, t57164)
}
