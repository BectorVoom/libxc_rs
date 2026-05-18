//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1251/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1251<F: Float>(t49847: F, t6241: F, t6240: F, t858: F, t867: F, t3128: F, t44315: F, t11592: F, t13491: F, t2121: F, t337: F, t3772: F, t3791: F, t9119: F) -> (F, F, F, F, F) {
    let t49853 = t49847 * t6241;
    let t49857 = F::new(3.0) / F::new(8.0) * t6240 * t867 * t858 * t49853;
    let t49859 = t3128 * t44315 / F::new(12.0);
    let t49861 = t11592 * t13491 / F::new(32.0);
    let t49875 = t9119 * t2121 * t337 * t3791 * t3772 / F::new(16.0);
    (t49853, t49857, t49859, t49861, t49875)
}
