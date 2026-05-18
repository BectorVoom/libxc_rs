//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 662/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk662<F: Float>(t280: F, t39: F, t2035: F, t1196: F, t817: F, t800: F, t4092: F, t10363: F, t5284: F, t5260: F, t816: F, t291: F) -> (F, F, F, F, F, F, F) {
    let t19038 = t280 * t39;
    let t19039 = t19038 * t2035;
    let t19048 = t817 * t1196;
    let t19049 = t800 * t19048;
    let t19053 = t4092 * t19048;
    let t19080 = t10363 * t5284;
    let t19095 = t816 * t5260;
    let t19100 = t291 * t39;
    (t19038, t19039, t19049, t19053, t19080, t19095, t19100)
}
