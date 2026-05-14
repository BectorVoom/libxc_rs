//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 696/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk696<F: Float>(t245: F, t10174: F, t1580: F, t21: F, t2624: F, t267: F, t363: F, t5: F, t7745: F, t776: F, t2961: F, t909: F, t332: F, t113: F, t2956: F, t4381: F, t2252: F, t342: F, t784: F) -> (F, F, F, F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t10188 = piecewise3(t246, 0.0, t5 * t10174 * t21 / 4.0 + 3.0 / 4.0 * t5 * t2624 * t363 + 3.0 / 4.0 * t5 * t776 * t1580 + t5 * t267 * t7745 / 4.0);
    let t10193 = t2961 * t909;
    let t10194 = t10193 * t332;
    let t10195 = t10194 * t113;
    let t10198 = t2956 * t909;
    let t10199 = t10198 * t4381;
    let t10207 = t342 * t2252 * t784 / 18.0;
    (t10188, t10194, t10195, t10198, t10199, t10207)
}
