//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1890/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1890<F: Float>(t12657: F, t487: F, t1209: F, t3727: F, t460: F, t12295: F, t1284: F, t3552: F, t1204: F, t3766: F) -> (F, F, F, F, F, F) {
    let t12658 = t12657 * t487;
    let t12666 = t1209 * t3727;
    let t12673 = t460 * t3727;
    let t12678 = F::cast_from(0.25925925925925925926e-1_f64) * t12295;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    (t12658, t12666, t12673, t12678, t12699, t12702)
}
