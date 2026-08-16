//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 880/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk880(t36042: f64, t871: f64, t1466: f64, t33808: f64, t35795: f64, t35799: f64, t35802: f64, t35810: f64, t35814: f64, t35817: f64, t36003: f64, t36005: f64, t36007: f64, t36009: f64, t36013: f64, t36017: f64, t6216: f64, t6963: f64, t6967: f64, t6972: f64, t7028: f64, t7581: f64, t7587: f64, t7618: f64) -> (f64, f64) {
    let t36043 = t871 * t36042;
    let t36047 = -t1466 * t35795 / 3.0_f64 + t1466 * t35799 - 2.0_f64 / 3.0_f64 * t1466 * t35802 - t7581 * t6972 / 3.0_f64 - t33808 * t6967 / 18.0_f64 - t6216 * t35810 / 18.0_f64 + t6216 * t35814 / 9.0_f64 + 4.0_f64 * t35817 + 2.0_f64 * t36003 - 4.0_f64 * t36005 - 2.0_f64 * t36007 - 4.0_f64 * t36009 - 2.0_f64 / 3.0_f64 * t1466 * t36013 + t1466 * t36017 / 6.0_f64 + t7581 * t7028 / 6.0_f64 - t6963 * t7587 / 3.0_f64 - 2.0_f64 * t36043 + t6963 * t7618 / 3.0_f64;
    (t36043, t36047)
}
