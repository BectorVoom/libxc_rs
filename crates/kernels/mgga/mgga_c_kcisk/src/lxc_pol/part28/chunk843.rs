//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 843/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk843<F: Float>(t140: F, t446: F, t728: F, t1925: F, t430: F, t1909: F, t574: F, t1860: F, t4597: F, t5438: F, t791: F, t10501: F, t1992: F, t772: F, t1961: F, t5372: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11885 = 0.11791604938271604938e-1 * t140 * t446 * t728;
    let t11894 = t140 * t430 * t1925;
    let t11900 = t1909 * t574;
    let t11905 = t1860 * t4597;
    let t11966 = 1.0 / t5438 / t791;
    let t11983 = 0.51588271604938271604e-3 * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = 1.0 / t11984;
    let t11986 = t772 * t11985;
    let t11999 = t1961 * t5372;
    (t11885, t11894, t11900, t11905, t11966, t11983, t11984, t11985, t11986, t11999)
}
