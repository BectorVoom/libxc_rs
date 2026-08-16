//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1647/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1647<F: Float>(t11151: F, t2908: F, t141: F, t11160: F, t930: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11328 = t2908 * t11151;
    let t11329 = t141 * t11328;
    let t11331 = t930 * t11160;
    let t11332 = t141 * t11331;
    let t11334 = F::cast_from(0.93011851851851851854e0_f64) * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = F::cast_from(0.36514074074074074075e0_f64) * t11337;
    let t11339 = t698 * t2909;
    (t11328, t11329, t11331, t11332, t11334, t11335, t11337, t11338, t11339)
}
