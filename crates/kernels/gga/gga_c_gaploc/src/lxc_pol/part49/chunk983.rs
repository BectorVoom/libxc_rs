//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 983/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk983<F: Float>(t2268: F, t8199: F, t9181: F, t2321: F, t34604: F, t9074: F, t29650: F, t2972: F, t13235: F, t14537: F, t8862: F, t9784: F) -> (F, F, F, F, F) {
    let t42896 = F::cast_from(0.14227503317838074799e1_f64) * t2268 * t9181 * t8199;
    let t42898 = t9074 * t34604 * t2321;
    let t42899 = F::cast_from(0.23712505529730124666e-2_f64) * t42898;
    let t42906 = F::cast_from(2.0_f64) * t29650 * t2972;
    let t42908 = F::cast_from(6.0_f64) * t14537 * t13235;
    let t42910 = F::cast_from(2.0_f64) * t8862 * t9784;
    (t42896, t42899, t42906, t42908, t42910)
}
