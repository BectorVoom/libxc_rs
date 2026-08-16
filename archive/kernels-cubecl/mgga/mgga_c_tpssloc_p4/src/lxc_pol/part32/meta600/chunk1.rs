//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1989/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1989<F: Float>(t1365: F, t1878: F, t22813: F, t6924: F, t80782: F, t22794: F, t22843: F, t281: F, t6597: F, t154: F, t8705: F, t1887: F, t534: F) -> (F, F, F, F, F, F) {
    let t80830 = t1878 * t1365;
    let t80836 = t22813 * t6924 * t80782;
    let t80837 = t80836 * t22794;
    let t80840 = t6597 * t22843 * t281;
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    (t80830, t80836, t80837, t80840, t80845, t80847)
}
