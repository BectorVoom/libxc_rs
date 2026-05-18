//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1021/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1021<F: Float>(t3204: F, t32350: F, t22958: F, t5674: F, t137224: F, t3188: F, t22953: F, t136269: F, t93351: F, t1871: F, t22952: F, t26006: F, t5675: F) -> (F, F, F, F, F, F, F) {
    let t144849 = t32350 * t3204;
    let t144851 = t5674 * t22958 * t144849;
    let t144853 = t137224 * t3188;
    let t144855 = t5674 * t22953 * t144853;
    let t144857 = t136269 * t3188;
    let t144859 = t5674 * t93351 * t144857;
    let t144863 = t22952 * t1871 * t5675 * t26006;
    (t144849, t144851, t144853, t144855, t144857, t144859, t144863)
}
