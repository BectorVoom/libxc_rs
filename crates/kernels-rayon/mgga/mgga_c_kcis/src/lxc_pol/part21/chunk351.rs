//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 351/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk351(t143: f64, t1697: f64, t1154: f64, t1155: f64, t1646: f64, t1745: f64, t304: f64, t1152: f64, t1153: f64, t1757: f64, t1761: f64, t348: f64, t365: f64, t368: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t1780 = t1697 * t143;
    let t1788 = t1154 * t1155 * t1646;
    let t1791 = t304 * t1745;
    let t1795 = 0.619125e-2_f64 * t1780 * t348 + 0.9286875e-2_f64 * t365 * t1757 - 0.619125e-2_f64 * t365 * t1761 - t1152 - 0.26531111111111111111e-1_f64 * t1153 * t1788 - 0.39796666666666666666e-1_f64 * t86 * t368 * t1791;
    (t1780, t1788, t1791, t1795)
}
