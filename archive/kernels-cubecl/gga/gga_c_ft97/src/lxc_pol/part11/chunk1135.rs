//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1135/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1135<F: Float>(t10577: F, t1775: F, t2778: F, t8282: F, t2767: F, t295: F, t41751: F, t10581: F, t10597: F, t3139: F, t849: F, t2775: F) -> (F, F, F, F, F, F, F, F) {
    let t43843 = t1775 * t10577;
    let t43848 = t8282 * t2778;
    let t43850 = t8282 * t2767;
    let t43852 = t41751 * t295;
    let t43860 = t1775 * t10581;
    let t43867 = t1775 * t10597;
    let t43872 = t3139 * t849;
    let t43874 = t8282 * t2775;
    (t43843, t43848, t43850, t43852, t43860, t43867, t43872, t43874)
}
