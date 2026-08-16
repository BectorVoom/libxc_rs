//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 791/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk791(t13772: f64, t13785: f64, t13804: f64, t13834: f64, t502: f64, t3749: f64, t977: f64, t1960: f64, t2592: f64, t123: f64, t3720: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13836 = t13772 + t13785 + t13804 + t13834;
    let t13837 = t502 * t13836;
    let t13838 = t3749 * t977;
    let t13839 = t1960 * t13838;
    let t13841 = t2592 * t3749;
    let t13846 = t3720 * t123;
    let t13847 = t13846 * t883;
    (t13836, t13837, t13838, t13839, t13841, t13846, t13847)
}
