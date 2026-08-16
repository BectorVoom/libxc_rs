//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 24/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk24(t44: f64, t48: f64, t47: f64, t43: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t49 = t48 * t44;
    let t50 = piecewise3(t45, t47, t49);
    let t51 = 1.0_f64 - t43;
    (t49, t50, t51)
}
