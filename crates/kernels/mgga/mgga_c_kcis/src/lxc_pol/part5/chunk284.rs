//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 284/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk284<F: Float>(t1035: F, t1045: F, t1027: F, t317: F, t319: F, t334: F, t240: F, t41: F) -> (F, F, F, F) {
    let t1046 = t1035 * t1045;
    let t1050 = 0.11955719325063177623e-1 * t1027;
    let t1055 = 0.3513e-2 * t317 * t334 * t319;
    let t1056 = t41 * t240;
    (t1046, t1050, t1055, t1056)
}
