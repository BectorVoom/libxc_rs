//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 185/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk185<F: Float>(t236: F, t551: F, t511: F, t558: F, t515: F, t570: F, t27: F, t29: F, t260: F) -> (F, F, F, F, F) {
    let t626 = t236 * t551;
    let t629 = t511 * t558;
    let t632 = t515 * t570;
    let t637 = t27 * t29;
    let t638 = t260 * t637;
    (t626, t629, t632, t637, t638)
}
