//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1143/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1143<F: Float>(t1501: F, t2843: F, t28924: F, t28963: F, t7581: F, t10688: F, t36060: F, t1248: F, t34053: F, t143002: F, t4181: F, t28873: F) -> (F, F, F, F, F, F) {
    let t153550 = t2843 * t1501 * t28924;
    let t153553 = t7581 * t28963;
    let t153555 = t10688 * t36060;
    let t153558 = t2843 * t34053 * t1248;
    let t153560 = t143002 * t4181;
    let t153567 = t7581 * t28873;
    (t153550, t153553, t153555, t153558, t153560, t153567)
}
