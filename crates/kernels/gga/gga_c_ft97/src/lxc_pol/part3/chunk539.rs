//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 539/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk539<F: Float>(t4246: F, t875: F, t296: F, t1248: F, t2749: F, t824: F, t992: F, t2875: F, t2874: F, t2882: F, t2881: F, t2360: F, t312: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4247 = t4246 * t875;
    let t4248 = t296 * t4247;
    let t4251 = t2749 * t1248;
    let t4252 = t296 * t4251;
    let t4255 = t992 * t824;
    let t4256 = t2875 * t4255;
    let t4257 = t2874 * t4256;
    let t4260 = t992 * t875;
    let t4261 = t2882 * t4260;
    let t4262 = t2881 * t4261;
    let t4265 = t312 * t2360;
    (t4247, t4248, t4251, t4252, t4256, t4257, t4261, t4262, t4265)
}
