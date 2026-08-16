//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1026/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1026(t108531: f64, t6757: f64, t35385: f64, t6050: f64, t30671: f64, t2035: f64, t35924: f64, t709: f64, t224: f64, t6793: f64, t9682: f64, t213: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150580 = t6757 * t108531;
    let t150590 = t35385 * t6050;
    let t150591 = t30671 * t150590;
    let t150594 = t2035 * t35924 * t709;
    let t150602 = t224 * t9682 * t6793;
    let t150603 = t665 * t213;
    (t150580, t150590, t150591, t150594, t150602, t150603)
}
