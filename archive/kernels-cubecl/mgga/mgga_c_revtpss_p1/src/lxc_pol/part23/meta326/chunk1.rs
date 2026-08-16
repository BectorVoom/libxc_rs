//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1623/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1623<F: Float>(t13847: F, t13848: F, t5675: F, t13845: F, t5609: F, t9794: F, t9793: F, t221: F, t5627: F, t9921: F, t3978: F, t2619: F, t5635: F) -> (F, F, F, F, F, F) {
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13877 = t221 * t5627;
    let t13878 = t9921 * t13877;
    let t13880 = F::cast_from(0.50820002809285328225e-3_f64) * t3978 * t13878;
    let t13887 = t5635 * t2619;
    (t13850, t13851, t13858, t13878, t13880, t13887)
}
