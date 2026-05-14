//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1245/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1245<F: Float>(t113459: F, t113508: F, t3699: F, t99528: F, t10478: F, t6318: F, t3690: F, t10248: F, t113222: F, t446: F, t113467: F, t2739: F, t7021: F, t1486: F, t193: F, t2781: F) -> (F, F, F, F, F, F) {
    let t113511 = t99528 * t113459 * t3699 * t113508;
    let t113513 = t10478 * t6318;
    let t113516 = t99528 * t113513 * t3690 * t113508;
    let t113519 = t446 * t10248 * t113222;
    let t113522 = t446 * t10248 * t113467;
    let t113524 = t7021 * t2739;
    let t113527 = t1486 * t193 * t2781 * t113524;
    (t113511, t113516, t113519, t113522, t113524, t113527)
}
