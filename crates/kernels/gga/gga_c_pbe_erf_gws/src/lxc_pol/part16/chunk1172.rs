//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1172/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1172<F: Float>(t15034: F, t859: F, t892: F, t1161: F, t353: F, t52191: F, t53952: F, t27729: F, t4082: F, t20154: F, t3067: F, t4207: F, t938: F, t14888: F, t15036: F, t19704: F, t20113: F, t29845: F, t52188: F, t52529: F, t53945: F, t53950: F, t53963: F, t53966: F, t53968: F, t6793: F, t8629: F, t8793: F) -> (F,) {
    let t55717 = t859 * t892 * t15034;
    let t55722 = t859 * t353 * t52191 * t1161;
    let t55726 = 7.0 / 144.0 * t53952;
    let t55729 = t27729 * t4082;
    let t55734 = t20154 * t3067 * t4207 * t938;
    let t55738 = t8629 * t52188 / 48.0 + t8793 * t52529 / 48.0 + t53945 / 128.0 + t19704 * t15036 / 48.0 + t19704 * t14888 / 48.0 + t20113 * t15036 / 48.0 + t6793 * t55717 / 24.0 + t6793 * t55722 / 24.0 + t53950 / 12.0 + t55726 + 5.0 / 192.0 * t53963 - t53966 / 24.0 - t29845 * t55729 / 32.0 - t6793 * t55734 / 12.0 + t53968 / 12.0;
    (t55738,)
}
