//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 688/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk688<F: Float>(t1018: F, t4992: F, t86: F, t1022: F, t4621: F, t1021: F, t1808: F, sigma0: F) -> (F, F, F, F, F) {
    let t4994 = t86 * t4992 * t1018;
    let t4995 = t1022 * t4621;
    let t4996 = t1021 * t4995;
    let t4997 = t4994 * t4996;
    let t4999 = t1808 * sigma0;
    (t4994, t4995, t4996, t4997, t4999)
}
