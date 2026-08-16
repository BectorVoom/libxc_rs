//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 762/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk762(t10627: f64, t22623: f64, t24885: f64, t787: f64, t2610: f64, t7291: f64, t10667: f64, t2089: f64, t321: f64, t3431: f64, t107: f64, t10012: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32847 = t22623 * t10627;
    let t32969 = t787 * t24885;
    let t33087 = t2610 * t7291;
    let t33118 = t2089 * t10667;
    let t33137 = t321 * t3431;
    let t33139 = t787 * t33137 * t107;
    let t33148 = t10012 * t10627;
    (t32847, t32969, t33087, t33118, t33137, t33139, t33148)
}
