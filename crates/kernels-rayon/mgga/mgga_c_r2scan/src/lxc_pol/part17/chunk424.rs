//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 424/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk424(t166: f64, t2055: f64, t2056: f64, t58: f64, t758: f64, t423: f64, t597: f64, t761: f64, t776: f64, t780: f64, t1267: f64, t261: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2059 = 0.571528e-1_f64 * t2055 * t166 * t2056;
    let t2060 = t758 * t58;
    let t2061 = t2060 * t423;
    let t2062 = t597 * t761;
    let t2063 = t2061 * t2062;
    let t2083 = t776 * t780;
    let t2086 = t261 * t1267 * t277;
    (t2059, t2060, t2061, t2062, t2063, t2083, t2086)
}
