//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 895/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk895<F: Float>(t8653: F, t8655: F, t9384: F, t9391: F, t8669: F, t9394: F, t9396: F, t9400: F, t9403: F, t9406: F, t10356: F, t37148: F, t8162: F, t8163: F, t8673: F, t9410: F) -> (F, F, F, F, F, F) {
    let t42485 = 0.5107751987195740728e-4 * t8653;
    let t42486 = 0.5987120850931904282e-1 * t8655;
    let t42487 = 0.11974241701863808564e0 * t9384;
    let t42489 = 0.11974241701863808564e0 * t9391;
    let t42491 = 0.212822999466489197e-4 * t8669;
    let t42492 = 0.39914139006212695214e-1 * t9394;
    let t42493 = 0.11974241701863808564e0 * t9396;
    let t42495 = 0.35922725105591425692e0 * t9400;
    let t42496 = 0.47896966807455234256e0 * t9403;
    let t42497 = 0.23948483403727617128e0 * t9406;
    let t42498 = t42491 + t37148 - t42492 - t8162 - t42493 - t8163 + 0.14546486215597515589e0 * t8673 + t42495 - t42496 - t42497 - t10356;
    let t42500 = 0.11974241701863808564e0 * t9410;
    (t42485, t42486, t42487, t42489, t42498, t42500)
}
