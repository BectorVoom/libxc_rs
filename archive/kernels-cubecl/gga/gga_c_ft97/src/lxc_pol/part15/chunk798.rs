//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 798/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk798<F: Float>(t1175: F, t2574: F, t4934: F, t1131: F, t5181: F, t729: F, t5053: F, t13927: F, t5064: F, t242: F, t21399: F, t265: F) -> (F, F, F, F, F, F) {
    let t21678 = t2574 * t1175 * t4934;
    let t21682 = t729 * t5181 * t1131;
    let t21686 = t729 * t1175 * t5053;
    let t21688 = t13927 * t5064;
    let t21689 = t242 * t21688;
    let t21693 = t729 * t265 * t21399;
    (t21678, t21682, t21686, t21688, t21689, t21693)
}
