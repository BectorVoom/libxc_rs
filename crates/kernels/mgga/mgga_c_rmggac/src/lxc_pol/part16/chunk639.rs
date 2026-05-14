//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 639/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk639<F: Float>(t262: F, t9888: F, t7641: F, t7648: F, t9885: F, t7653: F, t3826: F, t9708: F, t3851: F, t1707: F, t649: F, t7599: F, t7603: F, t117: F, t1704: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9889 = t262 * t9888;
    let t9890 = t7641 * t9889;
    let t9892 = t7648 * t9885;
    let t9894 = t7653 * t9889;
    let t9897 = t3826 * t9708;
    let t9899 = t3851 * t9708;
    let t9903 = t649 * t1707;
    let t9904 = t7599 * t9903;
    let t9906 = t7603 * t9903;
    let t9908 = t1704 * t117;
    (t9889, t9890, t9892, t9894, t9897, t9899, t9904, t9906, t9908)
}
