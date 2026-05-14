//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 93/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk93<F: Float>(t311: F, t312: F, t313: F, t303: F, t306: F, t309: F, t305: F) -> (F, F, F, F, F, F) {
    let t315 = t311 * t312 * t313;
    let t317 = 0.379785e1 * t306 + 0.8969e0 * t303 + 0.204775e0 * t309 + 0.123235e0 * t315;
    let t320 = 1.0 + 0.16081824322151104822e2 / t317;
    let t321 = f64::ln(t320);
    let t323 = 0.62182e-1 * t305 * t321;
    let t325 = 1.0 + 0.278125e-1 * t303;
    (t315, t317, t320, t321, t323, t325)
}
