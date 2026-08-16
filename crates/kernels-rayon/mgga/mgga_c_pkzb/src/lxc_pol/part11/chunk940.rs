//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 940/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk940(t2099: f64, t3886: f64, t2367: f64, t2382: f64, t3919: f64, t2381: f64, t2411: f64, t3730: f64, t824: f64, t758: f64, t1235: f64, t297: f64, t46: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10241 = t2099 * t3886;
    let t10242 = t2367 * t10241;
    let t10244 = t3919 * t2382;
    let t10245 = t2381 * t10244;
    let t10251 = t2411 * t3730;
    let t10252 = t10251 * t824;
    let t10253 = t758 * t10252;
    let t10257 = t1235 * t297 * t46;
    (t10241, t10242, t10244, t10245, t10251, t10252, t10253, t10257)
}
