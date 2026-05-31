//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 962/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk962<F: Float>(t11594: F, t11619: F, t1045: F, t373: F, t1042: F, t1034: F, t360: F, t11244: F, t11240: F, t3154: F, t357: F, t11249: F) -> (F, F, F, F, F, F) {
    let t11620 = t11594 + t11619;
    let t11622 = t373 * t11620 * t1045;
    let t11623 = t1042 * t11622;
    let t11626 = t1034 * t1034;
    let t11627 = F::cast_from(1.0_f64) / t11626;
    let t11628 = t11627 * t360;
    let t11629 = t11628 * t11244;
    let t11630 = t11240 * t11629;
    let t11631 = t3154 * t357;
    let t11632 = t11249 * t11631;
    (t11620, t11623, t11627, t11630, t11631, t11632)
}
