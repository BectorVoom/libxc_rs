//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1263/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1263<F: Float>(t31797: F, t32942: F, t36231: F, t36236: F, t36238: F, t37894: F, t37898: F, t37899: F, t37904: F, t37905: F, t37907: F, t37908: F, t37909: F, t37910: F, t37924: F, t40450: F, t40455: F, t40458: F) -> F {
    let t42118 = t37894 - t37898 - t37899 - t37904 + t37905 - t37907 - t37908 - t37909 - t37910 + F::new(0.18868855373762491241e-1) * t40450 - F::new(0.31448092289604152069e-3) * t31797 - t32942 - F::new(0.18140473443734395377e0) * t36231 + F::new(0.90702367218671976884e-1) * t36236 - F::new(0.38110238327173099531e-2) * t36238 - t37924 - F::new(0.42874018118069736972e-2) * t40455 - F::new(0.25724410870841842183e-2) * t40458;
    t42118
}
