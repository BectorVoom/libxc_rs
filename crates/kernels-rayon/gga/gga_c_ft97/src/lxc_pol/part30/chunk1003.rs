//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1003/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1003(t1882: f64, t35534: f64, t33319: f64, t3837: f64, t42500: f64, t6118: f64, t141357: f64, t2354: f64, t446: f64, t992: f64, t150096: f64, t150099: f64, t150102: f64, t150106: f64, t150109: f64, t150114: f64, t150118: f64, t150122: f64, t150125: f64, t150128: f64, t150131: f64, t150136: f64, t150139: f64) -> (f64, f64, f64, f64) {
    let t150140 = t1882 * t35534;
    let t150144 = t6118 * t42500 * t33319 * t3837;
    let t150148 = t446 * t2354 * t141357 * t992;
    let t150150 = -8.0_f64 / 3.0_f64 * t150096 + 4.0_f64 / 3.0_f64 * t150099 - 4.0_f64 / 3.0_f64 * t150102 - t150106 - 6.0_f64 * t150109 - 4.0_f64 / 3.0_f64 * t150114 + 2.0_f64 / 3.0_f64 * t150118 - 2.0_f64 / 3.0_f64 * t150122 - 2.0_f64 / 3.0_f64 * t150125 - 2.0_f64 / 3.0_f64 * t150128 - 8.0_f64 / 3.0_f64 * t150131 - 2.0_f64 * t150136 + t150139 - t150140 / 9.0_f64 + 12.0_f64 * t150144 + t150148 / 3.0_f64;
    (t150140, t150144, t150148, t150150)
}
