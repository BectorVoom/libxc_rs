//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 395/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk395(t171: f64, t1726: f64, t1727: f64, t1678: f64, t1683: f64, t1713: f64) -> (f64, f64, f64) {
    let t1730 = 0.1714584e0_f64 * t1726 * t171 * t1727;
    let t1731 = t1678 * t1683;
    let t1732 = t1731 * t1713;
    (t1730, t1731, t1732)
}
