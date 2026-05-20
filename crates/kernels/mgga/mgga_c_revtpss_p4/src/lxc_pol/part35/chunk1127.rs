//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1127/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1127<F: Float>(t10985: F, t26576: F, t2062: F, t2769: F, t786: F, t2070: F, t41154: F, t25876: F, t26304: F, t25894: F, t2097: F, t22: F, t25937: F) -> (F, F, F, F, F, F) {
    let t95930 = F::cast_from(0.46263278077393568556e-2_f64) * t26576 * t10985;
    let t95936 = t786 * t2062 * t2769;
    let t95964 = t2070 * t41154;
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96204 = t25937 * t2097 * t22;
    (t95930, t95936, t95964, t96186, t96187, t96204)
}
