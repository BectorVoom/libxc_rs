//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1248/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1248<F: Float>(t12832: F, t7413: F, t1599: F, t12651: F, t7429: F, t1610: F, t6176: F, t6177: F, t6183: F, t1601: F, t18431: F, t1600: F, t4425: F, t7421: F, t6141: F, t6148: F) -> (F, F, F, F, F, F) {
    let t23173 = t12832 * t7413;
    let t23174 = t1599 * t23173;
    let t23176 = t12651 * t7429;
    let t23177 = t23176 * t1610;
    let t23178 = t6176 * t23177;
    let t23181 = t6177 * t6183;
    let t23182 = t6176 * t23181;
    let t23185 = t1601 * t18431;
    let t23186 = t1600 * t23185;
    let t23191 = t4425 * t7421;
    let t23192 = t1599 * t23191;
    let t23194 = t6141 * t6148;
    (t23174, t23178, t23182, t23186, t23192, t23194)
}
