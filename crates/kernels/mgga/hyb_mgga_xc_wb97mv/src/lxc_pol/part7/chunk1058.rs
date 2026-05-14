//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1058/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1058<F: Float>(t10963: F, t10978: F, t6762: F, t6986: F, t8908: F, t9020: F, t284: F, t4166: F, t6786: F, t790: F, t3341: F, t3346: F, t2205: F, t4170: F, t6793: F, t8965: F) -> (F, F, F, F, F, F, F, F) {
    let t10980 = -t6986 + 0.12361111111111111111e-1 * t6762 + 0.24722222222222222223e-1 * t8908 - t9020 - 0.92708333333333333333e-2 * t10963 + 0.278125e-1 * t10978;
    let t10981 = t10980 * t284;
    let t10984 = t6786 * t4166;
    let t10985 = t10984 * t790;
    let t10987 = t3341 * t3346;
    let t10989 = t2205 * t4170;
    let t10990 = t10989 * t790;
    let t10995 = -t6793 + 4.0 / 9.0 * t6762 + 8.0 / 9.0 * t8908 - t8965 - t10963 / 3.0 + t10978;
    (t10980, t10981, t10984, t10985, t10987, t10989, t10990, t10995)
}
