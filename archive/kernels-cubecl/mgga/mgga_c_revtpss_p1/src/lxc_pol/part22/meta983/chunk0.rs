//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3333/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3333<F: Float>(t15390: F, t15421: F, t11294: F, t19318: F, t11528: F, t19321: F, t19324: F, t41883: F, t11289: F, t6142: F, t19128: F, t2869: F) -> (F, F, F, F, F, F) {
    let t63218 = F::cast_from(0.64327917994770140268e2_f64) * t15421 * t15390;
    let t63220 = F::cast_from(12.0_f64) * t11294 * t19318;
    let t63222 = F::cast_from(8.0_f64) * t11528 * t19321;
    let t63224 = F::cast_from(0.19298375398431042081e3_f64) * t41883 * t19324;
    let t63226 = F::cast_from(1.0_f64) * t11289 * t6142;
    let t63228 = F::cast_from(2.0_f64) * t2869 * t19128;
    (t63218, t63220, t63222, t63224, t63226, t63228)
}
