//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 818/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk818<F: Float>(t1775: F, t3500: F, t12330: F, t2102: F, t12283: F, t12288: F, t9192: F, t3515: F, t1033: F, t8282: F, t1986: F, t3518: F, t9016: F) -> (F, F, F, F, F, F, F) {
    let t12839 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1775 * t3500;
    let t12840 = t2102 * t12330;
    let t12843 = t2102 * t12283;
    let t12846 = t9192 * t12288;
    let t12850 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t3515;
    let t12852 = t8282 * t1033;
    let t12855 = t9016 * t3518 * t1986;
    (t12839, t12840, t12843, t12846, t12850, t12852, t12855)
}
