//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1215/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1215(t13953: f64, t42520: f64, t44243: f64, t44245: f64, t47113: f64, t47115: f64, t47121: f64, t47785: f64, t47788: f64, t47790: f64, t47791: f64, t48241: f64, t617: f64) -> f64 {
    let t48250 = t13953 * t617 - t42520 - t44243 - t44245 + t47113 + t47115 + t47121 - t47785 + t47788 - t47790 - t47791 - t48241;
    t48250
}
