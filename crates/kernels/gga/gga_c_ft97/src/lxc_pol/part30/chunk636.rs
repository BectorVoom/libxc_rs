//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 636/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk636<F: Float>(t28983: F, t296: F, t29020: F, t24886: F, t4151: F, t7102: F, t8392: F, t15191: F, t6274: F, t29045: F, t28719: F, t319: F, t840: F, t25271: F, t4176: F, t15460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29101 = t296 * t28983;
    let t29104 = t296 * t29020;
    let t29107 = t24886 * t4151;
    let t29111 = t8392 * t7102;
    let t29113 = t15191 * t6274;
    let t29116 = t296 * t29045;
    let t29120 = t840 * t319 * t28719;
    let t29123 = t25271 * t4176;
    let t29124 = t15460 * t29123;
    (t29101, t29104, t29107, t29111, t29113, t29116, t29120, t29123, t29124)
}
