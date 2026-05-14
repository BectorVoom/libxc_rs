//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 693/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk693<F: Float>(t371: F, t4141: F, t3983: F, t1134: F, t3990: F, t3991: F, t3989: F, t1162: F, t1178: F, t1177: F, t1105: F, t1192: F) -> (F, F, F, F, F, F, F) {
    let t4142 = t371 * t4141;
    let t4143 = t3983 * t4142;
    let t4146 = t3990 * t3991 * t1134;
    let t4147 = t3989 * t4146;
    let t4149 = t1178 * t1162;
    let t4150 = t371 * t4149;
    let t4151 = t1177 * t4150;
    let t4155 = t1192 * t1105;
    (t4142, t4143, t4146, t4147, t4150, t4151, t4155)
}
