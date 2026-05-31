//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1678/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1678<F: Float>(t1045: F, t11620: F, t373: F, t1042: F, t1034: F, t360: F, t11244: F, t11240: F, t3154: F, t357: F) -> (F, F, F, F, F, F, F) {
    let t11622 = t373 * t11620 * t1045;
    let t11623 = t1042 * t11622;
    let t11626 = t1034 * t1034;
    let t11627 = F::cast_from(1.0_f64) / t11626;
    let t11628 = t11627 * t360;
    let t11629 = t11628 * t11244;
    let t11630 = t11240 * t11629;
    let t11631 = t3154 * t357;
    (t11622, t11623, t11626, t11627, t11629, t11630, t11631)
}
