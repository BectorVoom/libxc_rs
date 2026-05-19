//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1246/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1246<F: Float>(t2610: F, t7291: F, t20019: F, t8775: F, t10978: F, t5771: F, t20671: F, t24501: F, t28309: F, t10847: F, t22706: F, t7584: F) -> (F, F, F, F) {
    let t33087 = t2610 * t7291;
    let t33090 = F::cast_from(0.55611873258433997041e0_f64) * t8775 * t20019 * t33087;
    let t33092 = F::cast_from(0.14300195980740170668e1_f64) * t5771 * t10978;
    let t33094 = t28309 * t20671 * t24501;
    let t33095 = F::cast_from(0.17041300423964777634e0_f64) * t33094;
    let t33098 = F::cast_from(0.30674340763136599742e2_f64) * t7584 * t22706 * t10847;
    (t33090, t33092, t33095, t33098)
}
