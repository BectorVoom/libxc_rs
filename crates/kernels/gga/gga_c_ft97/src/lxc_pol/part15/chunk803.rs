//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 803/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk803<F: Float>(t1175: F, t2594: F, t4965: F, t14159: F, t5166: F, t1091: F, t18680: F, t2606: F, t1168: F, t4917: F, t9808: F, t3891: F) -> (F, F, F, F, F, F, F) {
    let t21740 = t2594 * t1175 * t4965;
    let t21744 = t14159 * t5166;
    let t21747 = t18680 * t1091;
    let t21748 = t2606 * t21747;
    let t21752 = t4917 * t1168;
    let t21753 = t9808 * t21752;
    let t21754 = t3891 * t21753;
    (t21740, t21744, t21747, t21748, t21752, t21753, t21754)
}
