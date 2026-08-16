//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1155/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1155(t3281: f64, t842: f64, t877: f64, t10755: f64, t1882: f64, t10662: f64, t681: f64, t89: f64, t309: f64, t43833: f64, t870: f64, t9570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44318 = t3281 * t842;
    let t44320 = t3281 * t877;
    let t44330 = t1882 * t10755;
    let t44333 = t89 * t681 * t10662;
    let t44335 = t43833 * t309;
    let t44340 = t870 * t9570;
    (t44318, t44320, t44330, t44333, t44335, t44340)
}
