//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1130/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1130<F: Float>(t13847: F, t2684: F, t7354: F, t43783: F, t43787: F, t43790: F, t43793: F, t43800: F, t43803: F, t43806: F, t43809: F, t43812: F, t43815: F, t43817: F) -> F {
    let t47389 = t2684 * t7354 * t13847;
    let t47394 = -t43783 - F::cast_from(0.25561950635947166451e0_f64) * t47389 - t43787 + t43790 + t43793 + t43800 - t43803 + t43806 - t43809 - F::cast_from(0.14896037479937677779e-1_f64) * t43812 + F::cast_from(0.46011511144704899612e1_f64) * t43815 - F::cast_from(0.14896037479937677779e-1_f64) * t43817;
    t47394
}
