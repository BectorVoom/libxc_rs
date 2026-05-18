//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 890/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk890<F: Float>(t574: F, t638: F, t639: F, t8849: F, t1656: F, t2338: F, t1550: F, t2060: F, t30400: F, t194: F, t1979: F, t1982: F, t201: F, t6070: F) -> (F, F, F, F) {
    let t44925 = t638 * t639 * t8849 * t574;
    let t44929 = t638 * t639 * t2338 * t1656;
    let t44941 = t1550 * t2060 * t30400;
    let t44949 = t194 * t6070 * t201 * t1979 * t1982;
    (t44925, t44929, t44941, t44949)
}
