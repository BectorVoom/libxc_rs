//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1156/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1156<F: Float>(t4185: F, t840: F, t14423: F, t875: F, t13796: F, t3989: F, t1133: F, t898: F, t13798: F, t3214: F, t3959: F, t14121: F, t3209: F) -> (F, F, F, F, F, F, F, F) {
    let t14718 = t840 * t4185;
    let t14720 = t14423 * t875;
    let t14721 = t13796 * t14720;
    let t14722 = t3989 * t14721;
    let t14724 = t898 * t1133;
    let t14725 = t14724 * t13798;
    let t14726 = t13796 * t14725;
    let t14727 = t3989 * t14726;
    let t14729 = t3959 * t3214;
    let t14731 = t14121 * t3209;
    (t14718, t14721, t14722, t14724, t14726, t14727, t14729, t14731)
}
