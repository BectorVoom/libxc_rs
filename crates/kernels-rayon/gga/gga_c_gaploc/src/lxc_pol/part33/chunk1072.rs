//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1072/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1072(t2925: f64, t701: f64, t2610: f64, t24446: f64, t795: f64, t8720: f64, t313: f64, t769: f64, t8637: f64, t10007: f64, t8502: f64, t10012: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24451 = t2925 * t701;
    let t24474 = t2610 * t24451;
    let t24478 = t2610 * t24446;
    let t24487 = t795 * t8720;
    let t24488 = t313 * t24487;
    let t24496 = t769 * t8637;
    let t24501 = t10007 * t8502;
    let t24505 = t10012 * t8502;
    (t24474, t24478, t24487, t24488, t24496, t24501, t24505)
}
