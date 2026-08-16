//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 570/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk570<F: Float>(t25153: F, t1882: F, t6388: F, t6386: F, t870: F, t6224: F, t681: F, t6222: F, t683: F) -> (F, F, F, F, F) {
    let t25351 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t25153;
    let t25366 = t1882 * t6388;
    let t25368 = t870 * t6386;
    let t25409 = t681 * t6224;
    let t25412 = t683 * t6222;
    (t25351, t25366, t25368, t25409, t25412)
}
