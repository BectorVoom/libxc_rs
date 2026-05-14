//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 478/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk478<F: Float>(t5818: F, t5820: F, t3392: F, t2057: F, t6: F, t8: F) -> (F, F, F, F, F) {
    let t5821 = t5818 * t5820;
    let t5824 = t3392 * t5820;
    let t5827 = t2057 * t6;
    let t5828 = t5827 * t8;
    let t5829 = t3392 * t5828;
    (t5821, t5824, t5827, t5828, t5829)
}
