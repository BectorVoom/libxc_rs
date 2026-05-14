//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 814/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk814<F: Float>(t2252: F, t342: F, t4645: F, t4857: F, t8640: F, t4865: F, t39448: F, t4861: F, t4874: F, t4885: F, t4869: F, t2360: F, t2567: F, t2347: F, t5070: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t64677 = t342 * t2252 * t4645;
    let t64926 = t8640 * t4857;
    let t64969 = t8640 * t4865;
    let t64985 = t39448 * t4861;
    let t65113 = t8640 * t4874;
    let t65166 = t8640 * t4885;
    let t65258 = t8640 * t4869;
    let t65307 = t2567 * t2360;
    let t65313 = t2567 * t2347;
    let t65327 = t8232 * t5070;
    (t64677, t64926, t64969, t64985, t65113, t65166, t65258, t65307, t65313, t65327)
}
