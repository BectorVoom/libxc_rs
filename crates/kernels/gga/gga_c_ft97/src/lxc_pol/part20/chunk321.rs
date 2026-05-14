//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 321/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk321<F: Float>(t192: F, t2682: F, t2781: F, t2739: F, t852: F, t2761: F, t2762: F, t2764: F, t2767: F, t2772: F, t2775: F, t2778: F, t462: F, t92: F) -> (F, F, F) {
    let t2783 = t192 * t2781 * t2682;
    let t2787 = t192 * t852 * t2739;
    let t2789 = t2761 + 2.0 / 9.0 * t2762 + 2.0 / 3.0 * t2764 - 2.0 / 9.0 * t462 * t2767 + 2.0 / 3.0 * t462 * t2772 + 2.0 / 3.0 * t462 * t2775 - t462 * t2778 / 3.0 + 2.0 * t92 * t2783 - t92 * t2787;
    (t2783, t2787, t2789)
}
