//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 530/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk530<F: Float>(t332: F, t7137: F, t4: F, t6: F, t2: F, t2427: F, t39: F, t224: F, t173: F, t322: F, t674: F) -> (F, F, F, F, F, F, F) {
    let t7138 = t7137 * t332;
    let t7149 = t4 * t6;
    let t7242 = t2 * t2;
    let t7476 = t2427 * t39;
    let t7477 = t224 * t7476;
    let t7512 = t173 * t322;
    let t7513 = t674 * t674;
    let t7514 = 1.0 / t7513;
    (t7138, t7149, t7242, t7477, t7512, t7513, t7514)
}
