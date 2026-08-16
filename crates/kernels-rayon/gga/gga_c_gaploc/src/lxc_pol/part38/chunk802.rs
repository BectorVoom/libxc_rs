//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 802/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk802(t12797: f64, t1358: f64, t31591: f64, t4261: f64, t9074: f64, t2321: f64, t34600: f64, t12820: f64, t484: f64, t12770: f64, t2312: f64, t10590: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42673 = t1358 * t12797;
    let t42717 = t9074 * t4261 * t31591;
    let t42721 = t9074 * t34600 * t2321;
    let t42726 = t484 * t12820;
    let t42745 = t2312 * t12770;
    let t42748 = t882 * t10590 * t2321;
    (t42673, t42717, t42721, t42726, t42745, t42748)
}
