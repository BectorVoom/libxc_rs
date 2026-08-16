//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1063/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1063(t2723: f64, t8561: f64, t3932: f64, t3931: f64, t2725: f64, t2459: f64, t969: f64, t11476: f64, t11594: f64, t11598: f64, t11602: f64, t11609: f64, t2722: f64, t2740: f64, t3945: f64, t8559: f64, t8568: f64, t8989: f64, t9031: f64, t9033: f64, t9038: f64, t967: f64) -> f64 {
    let t11612 = t8561 * t2723;
    let t11613 = t3932 * t11612;
    let t11614 = t3931 * t11613;
    let t11617 = t3932 * t2725;
    let t11618 = t3931 * t11617;
    let t11621 = t969 * t2459;
    let t11622 = t11621 * t11476;
    let t11623 = t3931 * t11622;
    let t11628 = -t2740 * t11594 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t2740 * t11598 + t2740 * t11602 / 2304.0_f64 - t8989 * t3945 / 432.0_f64 + t2722 * t11609 / 1536.0_f64 + t8559 * t11614 / 512.0_f64 - t8568 * t11618 / 512.0_f64 + t967 * t11623 / 768.0_f64 + 19.0_f64 / 2592.0_f64 * t9031 + t9033 / 1296.0_f64 + t9038;
    t11628
}
