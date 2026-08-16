//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1259/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1259<F: Float>(t40425: F, t40428: F, t40434: F, t37468: F, t37477: F, t39075: F, t39076: F, t39081: F, t41786: F, t41788: F, t41790: F, t41794: F, t41797: F, t41800: F, t41803: F) -> F {
    let t42208 = F::cast_from(0.1440846329149835838e-2_f64) * t40425;
    let t42209 = F::cast_from(0.20496175532535769482e-3_f64) * t40428;
    let t42210 = F::cast_from(0.3842256877732895568e-2_f64) * t40434;
    let t42213 = -t39075 - t39076 + t42208 - t42209 - t41786 - t41788 - t41790 - t41794 + t42210 - F::cast_from(0.17347588262831798123e-3_f64) * t37468 - t39081 - F::cast_from(0.14088275218353950416e-1_f64) * t37477 - t41797 + t41800 - t41803;
    t42213
}
