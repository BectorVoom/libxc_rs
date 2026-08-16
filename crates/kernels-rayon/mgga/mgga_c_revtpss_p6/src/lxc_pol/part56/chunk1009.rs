//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1009/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1009(t1518: f64, t8964: f64, t1519: f64, t1911: f64, t33346: f64, t33578: f64, t33580: f64, t33583: f64, t33595: f64, t33599: f64, t33650: f64, t33654: f64, t33659: f64, t34377: f64, t34379: f64, t34383: f64, t34400: f64, t34401: f64, t34424: f64, t34880: f64, t569: f64, t651: f64, t7586: f64, t8158: f64, t8967: f64) -> (f64, f64) {
    let t34882 = t8964 * t1518;
    let t34886 = -2.0_f64 * t1519 * t33346 + t1911 * t8967 + t34880 * t569 - 2.0_f64 * t34882 * t651 - 4.0_f64 * t7586 * t8158 - t33578 - t33580 - t33583 - t33595 - t33599 - t33650 - t33654 + t33659 - 4.0_f64 * t34377 - 4.0_f64 * t34379 - 4.0_f64 * t34383 + 2.0_f64 * t34400 + 2.0_f64 * t34401 - 2.0_f64 * t34424;
    (t34882, t34886)
}
