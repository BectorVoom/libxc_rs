//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1078/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1078(t447: f64, t46867: f64, t1063: f64, t1064: f64, t11981: f64, t2293: f64, t2268: f64, t2343: f64, t42652: f64, t42655: f64, t42659: f64, t42661: f64, t42664: f64, t42671: f64, t42674: f64, t42675: f64, t42678: f64) -> (f64, f64, f64) {
    let t46941 = t46867 * t447;
    let t46944 = 0.28455006635676149599e-1_f64 * t1063 * t1064 * t46941;
    let t46945 = t11981 * t2293;
    let t46947 = t2268 * t2343 * t46945;
    let t46949 = -t42652 + t42655 - t42659 - 0.11856252764865062333e-2_f64 * t42661 + 0.11856252764865062333e-2_f64 * t42664 - 0.35568758294595186999e-2_f64 * t42671 - t42674 - 0.85365019907028448797e-1_f64 * t42675 - 0.85365019907028448797e-1_f64 * t42678 + t46944 + 0.56910013271352299198e-1_f64 * t46947;
    (t46941, t46945, t46949)
}
