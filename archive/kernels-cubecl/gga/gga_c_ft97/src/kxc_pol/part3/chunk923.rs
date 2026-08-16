//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 923/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk923<F: Float>(t18145: F, t18148: F, t505: F, t5165: F, t13683: F, t2506: F, t668: F, t713: F, t13689: F, t1775: F, t5102: F, t5110: F) -> (F, F, F, F, F, F, F) {
    let t18265 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18145;
    let t18266 = t18148 / F::cast_from(3.0_f64);
    let t18270 = t5165 * t505;
    let t18271 = t13683 * t18270;
    let t18274 = t2506 * t668;
    let t18276 = t18274 * t5165 * t713;
    let t18279 = t13689 * t18270;
    let t18283 = t1775 * t5102;
    let t18286 = t1775 * t5110;
    (t18265, t18266, t18271, t18276, t18279, t18283, t18286)
}
