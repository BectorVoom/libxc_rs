//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1235/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1235<F: Float>(t45140: F, t9016: F, t1109: F, t3825: F, t3855: F, t3065: F, t858: F, t8978: F, t11419: F, t45209: F, t44949: F, t13440: F, t2118: F, t3912: F, t860: F) -> (F, F, F, F, F, F, F) {
    let t49528 = t9016 * t45140 / F::new(4.0);
    let t49529 = t3825 * t1109;
    let t49534 = t3855 * t1109;
    let t49538 = t8978 * t3065 * t858 * t49534 / F::new(16.0);
    let t49540 = t11419 * t45209 / F::new(2.0);
    let t49545 = F::new(7.0) / F::new(12.0) * t44949;
    let t49550 = t3912 * t2118 * t13440 * t860 / F::new(24.0);
    (t49528, t49529, t49534, t49538, t49540, t49545, t49550)
}
