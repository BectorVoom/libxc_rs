//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 872/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk872<F: Float>(t2450: F, t7583: F, t8461: F, t1427: F, t1992: F, t7842: F, t1530: F, t1535: F, t30539: F, t2304: F, t7610: F, t1988: F, t8561: F, t8566: F, t1181: F, t4521: F, t604: F, t7426: F) -> (F, F, F, F, F, F, F) {
    let t34186 = t2450 * t7583 * t8461;
    let t34189 = t34186 * t7842 * t1992 * t1427;
    let t34204 = t1530 * t30539 * t1535;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34218 = 0.62896184579208304136e-3 * t34217;
    let t34221 = t1988 * t8566;
    let t34222 = 0.62896184579208304136e-3 * t34221;
    let t34237 = t7426 * t1181 * t604 * t4521;
    (t34186, t34189, t34204, t34215, t34218, t34222, t34237)
}
