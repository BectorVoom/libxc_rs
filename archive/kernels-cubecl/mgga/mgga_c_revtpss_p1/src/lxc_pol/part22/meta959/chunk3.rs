//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3220/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220<F: Float>(t49897: F, t4343: F, t890: F, t18871: F, t1940: F, t2403: F, t2408: F, t2832: F, t39442: F, t4556: F, t61031: F, t61032: F, t61033: F, t61039: F, t61088: F, t61091: F, t61094: F, t61097: F) -> (F, F) {
    let t61101 = F::cast_from(0.11696447245269292414e1_f64) * t49897;
    let t61102 = t4343 * t890;
    let t61106 = F::cast_from(2.0_f64) * t18871 * t1940 * t2832 + F::cast_from(2.0_f64) * t1940 * t2408 * t61033 - F::cast_from(12.0_f64) * t2403 * t4556 * t61102 + t39442 + t61031 + t61032 + t61039 + t61088 + t61091 - t61094 + t61097 - t61101;
    (t61101, t61106)
}
