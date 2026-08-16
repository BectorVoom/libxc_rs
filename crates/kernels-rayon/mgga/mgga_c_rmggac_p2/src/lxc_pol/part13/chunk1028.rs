//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1028/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1028(t8653: f64, t8655: f64, t9384: f64, t9391: f64, t8669: f64, t9394: f64, t9396: f64, t9400: f64, t9403: f64, t9406: f64, t10356: f64, t37148: f64, t8162: f64, t8163: f64, t8673: f64) -> (f64, f64, f64, f64, f64) {
    let t42485 = 0.5107751987195740728e-4_f64 * t8653;
    let t42486 = 0.5987120850931904282e-1_f64 * t8655;
    let t42487 = 0.11974241701863808564e0_f64 * t9384;
    let t42489 = 0.11974241701863808564e0_f64 * t9391;
    let t42491 = 0.212822999466489197e-4_f64 * t8669;
    let t42492 = 0.39914139006212695214e-1_f64 * t9394;
    let t42493 = 0.11974241701863808564e0_f64 * t9396;
    let t42495 = 0.35922725105591425692e0_f64 * t9400;
    let t42496 = 0.47896966807455234256e0_f64 * t9403;
    let t42497 = 0.23948483403727617128e0_f64 * t9406;
    let t42498 = t42491 + t37148 - t42492 - t8162 - t42493 - t8163 + 0.14546486215597515589e0_f64 * t8673 + t42495 - t42496 - t42497 - t10356;
    (t42485, t42486, t42487, t42489, t42498)
}
