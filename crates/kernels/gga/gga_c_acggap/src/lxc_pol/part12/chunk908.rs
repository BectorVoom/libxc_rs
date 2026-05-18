//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 908/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk908<F: Float>(t429: F, t7457: F, t7458: F, t7459: F, t3378: F, t7432: F, t2074: F, t12726: F, t2067: F, t2070: F, t1190: F, t30644: F) -> (F, F, F, F, F, F) {
    let t30790 = t7457 * t7458 * t429 * t7459;
    let t30792 = t3378 * t7432;
    let t30793 = t30792 * t2074;
    let t30797 = t12726 * t2067;
    let t30798 = t30797 * t2070;
    let t30800 = t30644 * t1190;
    (t30790, t30792, t30793, t30797, t30798, t30800)
}
