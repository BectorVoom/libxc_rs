//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 851/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk851<F: Float>(t140943: F, t33941: F, t33942: F, t140927: F, t7006: F, t33934: F, t33935: F, t7009: F, t2035: F, t213: F) -> (F, F, F, F, F) {
    let t142815 = t33941 * t140943 * t33942;
    let t142818 = 0.20139801475612389137e-1 * t7006 * t140927;
    let t142820 = t33934 * t140943 * t33935;
    let t142823 = 0.20139801475612389137e-1 * t7009 * t140927;
    let t142832 = t2035 * t213;
    (t142815, t142818, t142820, t142823, t142832)
}
