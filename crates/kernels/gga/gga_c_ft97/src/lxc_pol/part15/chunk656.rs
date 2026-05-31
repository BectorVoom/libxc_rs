//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 656/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk656<F: Float>(t1882: F, t5161: F, t5157: F, t17720: F, t18145: F, t18148: F, t1775: F, t5102: F, t5110: F, t2: F, t4934: F, t5099: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18188 = t1882 * t5161;
    let t18190 = t1882 * t5157;
    let t18241 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17720;
    let t18265 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18145;
    let t18266 = t18148 / F::cast_from(3.0_f64);
    let t18283 = t1775 * t5102;
    let t18286 = t1775 * t5110;
    let t18293 = t2 * t4934;
    let t18303 = t1775 * t5099;
    (t18188, t18190, t18241, t18265, t18266, t18283, t18286, t18293, t18303)
}
