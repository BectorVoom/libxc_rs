//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 779/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk779(t376: f64, t7266: f64, t89: f64, t1882: f64, t7231: f64, t28: f64, t32542: f64, t32547: f64, t32551: f64, t32555: f64, t32559: f64, t32564: f64, t32568: f64, t32573: f64, t32577: f64, t32581: f64, t446: f64) -> (f64, f64, f64) {
    let t32587 = t89 * t376 * t7266 / 9.0_f64;
    let t32589 = 2.0_f64 / 9.0_f64 * t1882 * t7231;
    let t32590 = 4.0_f64 / 3.0_f64 * t446 * t32542 - t446 * t32547 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t32551 + 2.0_f64 / 3.0_f64 * t446 * t32555 - 2.0_f64 * t446 * t32559 - 2.0_f64 / 3.0_f64 * t446 * t32564 + 4.0_f64 / 3.0_f64 * t446 * t32568 + 2.0_f64 / 3.0_f64 * t446 * t32573 - t446 * t32577 / 9.0_f64 + t89 * t28 * t32581 / 3.0_f64 - t32587 - t32589;
    (t32587, t32589, t32590)
}
