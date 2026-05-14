//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 846/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk846<F: Float>(t19106: F, t285: F, t4089: F, t4092: F, t4061: F, t5261: F, t274: F, t3750: F, t18010: F, t683: F, t17894: F, t231: F, t5249: F, t8959: F, t4939: F, t703: F) -> (F, F, F, F, F, F, F, F) {
    let t19135 = t285 * t19106;
    let t19144 = t4092 * t4089;
    let t19147 = t4061 * t5261;
    let t19151 = t274 * t3750;
    let t19155 = t683 * t18010 * t274;
    let t19162 = t17894 * t274;
    let t19163 = t231 * t19162;
    let t19167 = 0.8854768453090786061e-3 * t8959 * t5249;
    let t19168 = t703 * t4939;
    (t19135, t19144, t19147, t19151, t19155, t19163, t19167, t19168)
}
