//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1077/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1077(t2055: f64, t27060: f64, t29432: f64, t32176: f64, t32178: f64, t32642: f64, t32644: f64, t32646: f64, t32654: f64, t32657: f64, t32659: f64, t33286: f64, t33287: f64, t670: f64, t7373: f64, t7586: f64, t8564: f64) -> f64 {
    let t33296 = 2.0_f64 * t2055 * t27060 + 2.0_f64 * t2055 * t29432 + 2.0_f64 * t33287 * t670 + 2.0_f64 * t7373 * t7586 + t32176 + t32178 + t32642 + t32644 + t32646 + t32654 + t32657 + t32659 + t33286 + t8564;
    t33296
}
