//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 381/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk381(t236: f64, t2627: f64, t240: f64, t812: f64, t232: f64) -> (f64, f64, f64) {
    let t2628 = t2627 * t236;
    let t2629 = t2628 * t240;
    let t2630 = t812 * t2629;
    let t2632 = t232 * t232;
    (t2628, t2630, t2632)
}
