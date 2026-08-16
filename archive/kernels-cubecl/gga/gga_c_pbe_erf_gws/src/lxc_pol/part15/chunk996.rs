//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 996/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk996<F: Float>(t3106: F, t6472: F, t8782: F, t860: F, t3116: F, t6707: F, t1105: F, t2182: F) -> (F, F, F) {
    let t8933 = t6472 * t3106;
    let t8934 = t8782 * t8933;
    let t8936 = t8934 * t860 / F::cast_from(96.0_f64);
    let t8938 = t3116 * t6707 / F::cast_from(96.0_f64);
    let t8939 = t1105 * t2182;
    (t8936, t8938, t8939)
}
