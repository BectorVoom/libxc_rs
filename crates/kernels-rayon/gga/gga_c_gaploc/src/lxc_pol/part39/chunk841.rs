//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 841/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk841(t10627: f64, t22623: f64, t24885: f64, t787: f64, t1457: f64, t2634: f64, t2610: f64, t7291: f64, t10667: f64, t2089: f64, t321: f64, t3431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32847 = t22623 * t10627;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t33087 = t2610 * t7291;
    let t33118 = t2089 * t10667;
    let t33137 = t321 * t3431;
    (t32847, t32969, t32970, t33087, t33118, t33137)
}
