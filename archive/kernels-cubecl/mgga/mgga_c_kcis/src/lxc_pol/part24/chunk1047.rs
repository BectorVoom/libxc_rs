//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1047/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1047<F: Float>(t1121: F, t1800: F, t27763: F, t1092: F, t1133: F, t14628: F, t26760: F, t2909: F, t417: F) -> (F, F, F, F, F, F, F) {
    let t27764 = t1800 * t1121;
    let t27765 = t27763 * t27764;
    let t27766 = t1092 * t27765;
    let t27768 = t14628 * t1133;
    let t27769 = t26760 * t27768;
    let t27770 = t1092 * t27769;
    let t27772 = t417 * t2909;
    (t27764, t27765, t27766, t27768, t27769, t27770, t27772)
}
