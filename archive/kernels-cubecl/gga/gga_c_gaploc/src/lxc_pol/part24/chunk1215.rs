//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1215/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1215<F: Float>(t10770: F, t7137: F, t2958: F, t7112: F, t2508: F, t2580: F, t3431: F, t723: F) -> (F, F, F, F) {
    let t32207 = F::cast_from(0.20508069947045931424e-1_f64) * t7137 * t10770;
    let t32210 = t2958 * t7112;
    let t32213 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t32210;
    let t32214 = t3431 * t723;
    (t32207, t32210, t32213, t32214)
}
