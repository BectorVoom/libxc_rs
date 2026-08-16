//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 671/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk671(t10846: f64, t10874: f64, t10902: f64, t10945: f64, t10991: f64, t11036: f64, t11087: f64, t11122: f64, t3511: f64, t841: f64, t3073: f64, t977: f64) -> (f64, f64, f64) {
    let t11125 = t10846 + t10874 + t10902 + t10945 + t10991 + t11036 + t11087 + t11122;
    let t11127 = t3511 * t841;
    let t11135 = t3073 * t977;
    (t11125, t11127, t11135)
}
