//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1083/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1083<F: Float>(t16901: F, t24651: F, t24653: F, t20353: F, t10534: F, t46: F, t552: F, t16810: F, t16813: F, t16906: F, t16909: F, t16915: F, t16923: F, t20349: F, t20352: F, t20359: F, t20360: F, t20363: F) -> (F, F, F, F, F, F) {
    let t29131 = 0.51947577317044391277e2 * t16901;
    let t29132 = 12.0 * t24651;
    let t29133 = 24.0 * t24653;
    let t29134 = 0.30762056574649219972e4 * t20353;
    let t29136 = t10534 * t46 * t552;
    let t29137 = 0.18311447306006545054e-3 * t29136;
    let t29138 = -t29131 - t16906 + t16909 - t20349 + t16915 - t16923 - t29132 - t29133 + t20352 - t29134 - t20359 + t20360 - t20363 - t29137 + t16810 - t16813;
    (t29131, t29132, t29133, t29134, t29137, t29138)
}
