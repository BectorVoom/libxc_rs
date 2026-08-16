//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2933/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2933<F: Float>(t11710: F, t15600: F, t3091: F, t127: F, t4823: F, t11774: F, t3096: F, t11675: F, t15592: F, t15596: F, t42121: F, t42122: F, t42124: F, t42139: F, t42141: F, t42146: F, t42149: F) -> F {
    let t53389 = t3091 * t11710 * t15600;
    let t53391 = t127 * t4823;
    let t53393 = t11774 * t53391 * t3096;
    let t53395 = -t42121 - F::cast_from(0.22866142996303859718e-2_f64) * t42122 - F::cast_from(0.14291339372689912324e-3_f64) * t42124 - F::cast_from(0.35400176935018568009e-1_f64) * t42139 - F::cast_from(0.48272968547752592738e-2_f64) * t42141 + F::cast_from(0.14481890564325777822e-1_f64) * t42146 + F::cast_from(0.14291339372689912324e-3_f64) * t42149 + F::cast_from(0.42874018118069736972e-3_f64) * t11675 * t15592 + F::cast_from(0.7145669686344956162e-3_f64) * t11675 * t15596 + F::cast_from(0.28582678745379824648e-3_f64) * t53389 - F::cast_from(0.57165357490759649295e-3_f64) * t53393;
    t53395
}
