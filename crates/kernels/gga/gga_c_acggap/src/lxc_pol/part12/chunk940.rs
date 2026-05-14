//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 940/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk940<F: Float>(t17752: F, t2030: F, t8919: F, t2068: F, t7342: F, t8480: F, t4680: F, t8738: F, t1181: F, t20987: F, t7351: F, t7575: F, t20992: F, t7426: F, t20138: F, t599: F, t7413: F) -> (F, F, F, F, F, F) {
    let t34929 = t2030 * t17752 * t8919;
    let t34933 = t2068 * t8480 * t7342;
    let t34937 = t2068 * t4680 * t8738;
    let t34941 = t7575 * t1181 * t7351 * t20987;
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34949 = t7413 * t1181 * t599 * t20138;
    (t34929, t34933, t34937, t34941, t34945, t34949)
}
