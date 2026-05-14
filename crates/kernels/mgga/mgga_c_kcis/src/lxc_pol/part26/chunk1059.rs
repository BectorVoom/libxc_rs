//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1059/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1059<F: Float>(t2104: F, t2109: F, t27614: F, t6176: F, t6912: F, t7979: F, t1600: F, t27595: F, t28714: F, t28727: F, t28742: F, t29267: F, t29271: F, t29275: F, t29278: F, t29281: F, t29510: F, t29514: F, t7968: F, t7978: F, t8222: F, t8226: F) -> (F, F, F, F, F, F) {
    let t29524 = t2109 * t2104;
    let t29525 = t27614 * t29524;
    let t29526 = t6176 * t29525;
    let t29532 = t7979 * t6912;
    let t29533 = t1600 * t29532;
    let t29540 = 0.34752604166666666667e-3 * t7978 * t29510 - 0.92835860883789062501e-5 * t27595 * t29514 - 0.18534722222222222222e-2 * t28727 * t8226 - 0.69505208333333333334e-3 * t7978 * t29514 + 0.23214722222222222222e-2 * t29267 + 0.11607361111111111111e-2 * t29271 + 0.19345601851851851852e-2 * t29275 - 0.92754700520833333334e-4 * t7968 * t29526 - 0.7722800925925925926e-4 * t28742 - 0.23214722222222222222e-2 * t29278 + 0.15476481481481481481e-2 * t29281 + 0.23168402777777777778e-3 * t7978 * t29533 - 0.23168402777777777778e-3 * t28714 * t8222 - 0.69505208333333333334e-3 * t7978 * t29526;
    (t29524, t29525, t29526, t29532, t29533, t29540)
}
