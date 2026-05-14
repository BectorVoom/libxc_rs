//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 527/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk527<F: Float>(t103: F, t4545: F, t108: F, t4415: F, t4501: F, t4552: F, t4590: F, t4594: F, t4621: F, t88: F, t948: F, t984: F, t4431: F) -> (F, F, F) {
    let t4623 = t4545 * t103;
    let t4628 = -t108 * t4415 - t108 * t4501 - t4621 * t88 - 2.0 * t948 * t984 + 4.0 * t4552 - 2.0 * t4590 - 4.0 * t4594 + 2.0 * t4623;
    let t4635 = -t4431;
    (t4623, t4628, t4635)
}
