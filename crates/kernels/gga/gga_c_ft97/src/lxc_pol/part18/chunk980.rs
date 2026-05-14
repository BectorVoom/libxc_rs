//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 980/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk980<F: Float>(t25591: F, t83: F, t3238: F, t452: F, t5722: F, t25593: F, t1332: F, t3103: F, t488: F, t23294: F, t925: F, t1909: F, t23323: F, t3183: F, t1882: F, t6549: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26230 = t83 * t25591;
    let t26234 = t452 * t3238 * t5722;
    let t26237 = t83 * t25593;
    let t26240 = t1332 * t3103;
    let t26242 = t452 * t488 * t26240;
    let t26245 = t23294 * t925;
    let t26246 = t1909 * t26245;
    let t26249 = t23323 * t3183;
    let t26252 = t1882 * t6549;
    (t26230, t26234, t26237, t26240, t26242, t26245, t26246, t26249, t26252)
}
