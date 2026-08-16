//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 965/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk965<F: Float>(t34217: F, t1988: F, t8566: F, t1181: F, t4521: F, t604: F, t7426: F, t1466: F, t30644: F, t137: F, t14423: F, t30209: F, t5099: F) -> (F, F, F, F, F, F) {
    let t34218 = F::cast_from(0.62896184579208304136e-3_f64) * t34217;
    let t34221 = t1988 * t8566;
    let t34222 = F::cast_from(0.62896184579208304136e-3_f64) * t34221;
    let t34237 = t7426 * t1181 * t604 * t4521;
    let t34239 = t30644 * t1466;
    let t34240 = F::cast_from(0.17149607247227894789e-2_f64) * t34239;
    let t34248 = t14423 * t137;
    let t34255 = t30209 * t1181 * t604 * t5099;
    (t34218, t34222, t34237, t34240, t34248, t34255)
}
