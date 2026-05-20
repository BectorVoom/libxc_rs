//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2570/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570<F: Float>(t11262: F, t1247: F, t5286: F, t13099: F, t43776: F, t12909: F, t17395: F, t44546: F, t5331: F, t5334: F, t13032: F, t17528: F) -> (F, F, F, F, F) {
    let t57125 = t1247 * t11262 * t5286;
    let t57126 = F::cast_from(0.14291339372689912324e-3_f64) * t57125;
    let t57136 = t13099 * t43776;
    let t57147 = t12909 * t17395;
    let t57222 = t5331 * t44546 * t5334;
    let t57223 = F::cast_from(0.14291339372689912324e-3_f64) * t57222;
    let t57229 = t13032 * t17528;
    (t57126, t57136, t57147, t57223, t57229)
}
