//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 809/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk809<F: Float>(t9708: F, t9977: F, t6719: F, t748: F, t1873: F, t2580: F, t1800: F, t2587: F, t2591: F, t9970: F, t9973: F, t9975: F) -> (F, F, F, F, F, F) {
    let t9978 = t9708 * t9977;
    let t9980 = t6719 * t748;
    let t9982 = t1873 * t2580;
    let t9984 = t1800 * t2587;
    let t9986 = t1800 * t2591;
    let t9988 = t9970 / 16.0 - t9973 / 16.0 - t9975 / 6.0 + t9978 / 24.0 - t9980 / 128.0 + t9982 / 128.0 + t9984 / 24.0 - t9986 / 96.0;
    (t9978, t9980, t9982, t9984, t9986, t9988)
}
