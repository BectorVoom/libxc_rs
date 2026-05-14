//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1013/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1013<F: Float>(t26863: F, t3441: F, t1901: F, t23425: F, t23427: F, t26826: F, t26830: F, t26833: F, t26838: F, t26842: F, t26846: F, t26851: F, t26854: F, t26857: F, t26860: F, t446: F) -> (F, F) {
    let t26864 = t26863 * t3441;
    let t26867 = -2.0 / 9.0 * t26826 - t23425 / 9.0 - 2.0 / 9.0 * t23427 + 2.0 / 3.0 * t446 * t26830 + 2.0 / 3.0 * t446 * t26833 + t446 * t26838 / 3.0 + t446 * t26842 / 3.0 + t446 * t26846 / 3.0 + t446 * t26851 / 3.0 + t1901 * t26854 / 9.0 + t1901 * t26857 / 9.0 + 2.0 / 9.0 * t1901 * t26860 - 2.0 / 27.0 * t1901 * t26864;
    (t26864, t26867)
}
