//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 791/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk791<F: Float>(t24793: F, t2609: F, t2409: F, t6161: F, t2606: F, t242: F, t24410: F, t24416: F, t1456: F, t2373: F, t2574: F, t24403: F, t1431: F, t8232: F, t2405: F, t6074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24794 = t24793 * t2609;
    let t24797 = t6161 * t2409;
    let t24798 = t2606 * t24797;
    let t24801 = t242 * t24410;
    let t24804 = t242 * t24416;
    let t24808 = t2574 * t1456 * t2373;
    let t24811 = t242 * t24403;
    let t24815 = 4.0 / 27.0 * t8232 * t1431;
    let t24816 = t6074 * t2405;
    (t24794, t24797, t24798, t24801, t24804, t24808, t24811, t24815, t24816)
}
