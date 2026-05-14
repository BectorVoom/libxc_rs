//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 826/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk826<F: Float>(t4088: F, t816: F, t820: F, t2735: F, t4064: F, t2687: F, t283: F, t291: F, t287: F, t4061: F, t1471: F, t800: F, t1111: F, t1472: F, t14721: F, t14725: F, t14729: F, t14731: F, t14734: F, t14739: F, t14742: F, t14746: F, t14749: F, t2691: F, t2726: F, t4065: F, t4113: F) -> (F,) {
    let t14752 = t816 * t4088;
    let t14753 = t14752 * t820;
    let t14756 = t4064 * t2735;
    let t14759 = t2687 * t283;
    let t14760 = t14759 * t291;
    let t14763 = t4061 * t287;
    let t14766 = t800 * t1471;
    let t14769 = 0.2416365355361531912e1 * t14721 * t14725 + 0.1208182677680765956e1 * t14729 * t14731 - 6.0 * t4113 * t14734 * t2726 + 4.0 * t2691 * t14739 - 0.1208182677680765956e1 * t14742 * t14731 - 0.60409133884038297798e0 * t1472 * t14746 - 2.0 * t2691 * t14749 - 4.0 * t2691 * t14753 - 2.0 * t2691 * t14756 - 0.1208182677680765956e1 * t14760 * t1111 - 4.0 * t14763 * t4065 - 0.2416365355361531912e1 * t14766 * t14725;
    (t14769,)
}
