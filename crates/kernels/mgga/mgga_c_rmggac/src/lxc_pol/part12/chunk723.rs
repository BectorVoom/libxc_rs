//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 723/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk723<F: Float>(t875: F, t899: F, t117: F, t1540: F, t833: F, t321: F, t325: F, t570: F, t876: F, t1614: F, t352: F, t880: F) -> (F, F, F, F, F, F, F) {
    let t30204 = t899 * t875;
    let t30221 = t1540 * t117;
    let t30510 = t833 * t117;
    let t30526 = t321 * t325;
    let t30900 = t570 * t876;
    let t31043 = t1614 * t352;
    let t31057 = t899 * t880;
    (t30204, t30221, t30510, t30526, t30900, t31043, t31057)
}
