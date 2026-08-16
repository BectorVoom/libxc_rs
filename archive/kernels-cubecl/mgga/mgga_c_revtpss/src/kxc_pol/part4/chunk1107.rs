//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1107/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1107<F: Float>(t9552: F, t9559: F, t1317: F, t5567: F, t9564: F, t9566: F, t9578: F, t9580: F, t4147: F, t5778: F, t2496: F, t5571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13640 = F::cast_from(0.5848223622634646207e0_f64) * t9552;
    let t13641 = F::cast_from(40.0_f64) * t9559;
    let t13643 = F::cast_from(8.0_f64) * t1317 * t5567;
    let t13644 = F::cast_from(0.18311447306006545054e-3_f64) * t9564;
    let t13645 = F::cast_from(0.4883052614935078681e-3_f64) * t9566;
    let t13646 = F::cast_from(24.0_f64) * t9578;
    let t13647 = F::cast_from(4.0_f64) * t9580;
    let t13648 = t5778 * t4147;
    let t13652 = t5571 * t2496;
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652)
}
