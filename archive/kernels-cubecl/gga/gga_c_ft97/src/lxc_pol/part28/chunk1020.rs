//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1020/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1020<F: Float>(t136188: F, t136189: F, t25996: F, t32067: F, t22952: F, t22953: F, t25985: F, t32350: F, t34379: F, t379: F, t5674: F, t93355: F) -> (F, F, F, F) {
    let t144840 = t32067 * t136188 * t136189 * t25996;
    let t144844 = t22952 * t22953 * t32350 * t25985;
    let t144846 = t34379 * t379;
    let t144848 = t5674 * t93355 * t144846;
    (t144840, t144844, t144846, t144848)
}
