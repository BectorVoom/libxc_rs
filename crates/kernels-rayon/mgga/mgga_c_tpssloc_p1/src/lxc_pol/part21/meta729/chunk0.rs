//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2584/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2584(t15800: f64, t225: f64, t15808: f64, t14731: f64, t15419: f64, t3447: f64, t12606: f64, t3450: f64, t1714: f64, t44583: f64, t3451: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    let t51948 = t3447 * t15419 * t14731;
    let t51961 = t3450 * t12606;
    let t51968 = t44583 * t1714;
    let t51970 = t3447 * t51968 * t3451;
    let t51975 = t458 * t1714;
    (t51928, t51937, t51948, t51961, t51968, t51970, t51975)
}
