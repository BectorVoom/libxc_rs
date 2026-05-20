//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3108/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3108<F: Float>(t12784: F, t17451: F, t17416: F, t3647: F, t11262: F, t1247: F, t5286: F, t17501: F, t3172: F, t3711: F, t13099: F, t43776: F) -> (F, F, F, F, F) {
    let t57114 = t12784 * t17451;
    let t57118 = t3647 * t17416;
    let t57125 = t1247 * t11262 * t5286;
    let t57126 = F::cast_from(0.14291339372689912324e-3_f64) * t57125;
    let t57128 = t3711 * t3172 * t17501;
    let t57136 = t13099 * t43776;
    (t57114, t57118, t57126, t57128, t57136)
}
