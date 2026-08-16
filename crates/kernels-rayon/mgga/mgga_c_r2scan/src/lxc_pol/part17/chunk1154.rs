//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1154/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1154(t3295: f64, t9517: f64, t3308: f64, t37965: f64, t8821: f64, t37961: f64, t9501: f64, t1577: f64, t9508: f64, t6218: f64, t8803: f64, t11797: f64, t2651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42982 = t3295 * t9517;
    let t42985 = t37965 * t3308 * t8821;
    let t42988 = t37961 * t3308 * t9501;
    let t42991 = t1577 * t3308 * t9508;
    let t42994 = t6218 * t3308 * t8803;
    let t42996 = t2651 * t11797;
    (t42982, t42985, t42988, t42991, t42994, t42996)
}
