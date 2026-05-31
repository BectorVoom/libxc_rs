//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 991/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk991<F: Float>(t3459: F, t5552: F, t3073: F, t977: F, t1960: F, t2595: F, t8862: F, t2592: F, t3689: F, t555: F) -> (F, F, F, F, F, F) {
    let t11134 = F::cast_from(2.0_f64) * t5552 * t3459;
    let t11135 = t3073 * t977;
    let t11137 = F::cast_from(2.0_f64) * t1960 * t11135;
    let t11139 = F::cast_from(2.0_f64) * t8862 * t2595;
    let t11140 = t2592 * t3073;
    let t11977 = t555 * t3689;
    (t11134, t11135, t11137, t11139, t11140, t11977)
}
