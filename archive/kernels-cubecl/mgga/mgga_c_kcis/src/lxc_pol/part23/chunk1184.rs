//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1184/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1184<F: Float>(t16937: F, t27454: F, t7908: F, t27380: F, t4142: F, t11814: F, t27384: F, t12246: F, t491: F, t990: F, t27365: F, t94247: F) -> (F, F, F, F, F, F) {
    let t94310 = t7908 * t16937 * t27454;
    let t94319 = t4142 * t27380;
    let t94321 = t11814 * t27384;
    let t94331 = t12246 * t491 * t990;
    let t94340 = t4142 * t27365;
    let t94342 = t7908 * t94247;
    (t94310, t94319, t94321, t94331, t94340, t94342)
}
