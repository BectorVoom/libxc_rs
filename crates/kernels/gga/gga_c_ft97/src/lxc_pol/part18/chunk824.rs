//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 824/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk824<F: Float>(t22862: F, t369: F, t108: F, t28: F, t5495: F, t5498: F, t1309: F, t1637: F, t1286: F, t1586: F, t5617: F) -> (F, F, F, F, F, F, F) {
    let t22863 = t369 * t22862;
    let t22864 = t22863 * t108;
    let t22865 = t28 * t22864;
    let t22868 = t5495 * t5498;
    let t22870 = t1637 * t1309;
    let t22872 = 2.0 / 27.0 * t1286 * t22870;
    let t22873 = t1586 * t5617;
    (t22863, t22864, t22865, t22868, t22870, t22872, t22873)
}
