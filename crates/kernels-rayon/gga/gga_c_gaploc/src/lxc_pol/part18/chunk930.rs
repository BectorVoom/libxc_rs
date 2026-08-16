//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 930/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk930(t10140: f64, t2343: f64, t2268: f64, t2293: f64, t2787: f64) -> (f64, f64, f64) {
    let t10141 = t2343 * t10140;
    let t10143 = 0.56910013271352299198e-1_f64 * t2268 * t10141;
    let t10144 = t2787 * t2293;
    (t10141, t10143, t10144)
}
