//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 746/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk746<F: Float>(t1614: F, t664: F, t40331: F, t793: F, t558: F, t7817: F, t797: F, t305: F, t38381: F, t262: F, t40802: F, t7835: F, t39666: F, t7788: F, t40805: F, t7782: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40940 = t664 * t1614;
    let t40944 = t793 * t40331;
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40965 = t262 * t40802;
    let t40966 = t7835 * t40965;
    let t40967 = 0.36366215538993788972e-1 * t40966;
    let t40970 = t7788 * t39666;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    (t40940, t40944, t40948, t40949, t40951, t40965, t40967, t40970, t40975, t40976)
}
