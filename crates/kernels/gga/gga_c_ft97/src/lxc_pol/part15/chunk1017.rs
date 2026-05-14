//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1017/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1017<F: Float>(t1136: F, t21122: F, t258: F, t263: F, t5058: F, t5059: F, t5179: F, t661: F, t88131: F, t89092: F, t89097: F, t89179: F, t89442: F, t89465: F, t89547: F, t89565: F, t89685: F, t89704: F, t89712: F, t89727: F, t89741: F) -> (F,) {
    let t89749 = -3.0 * t21122 * t5058 * t263 - 8.0 * t89565 - 12.0 * t88131 + 48.0 * t89442 - 72.0 * t89465 - t89685 * t1136 * t263 - 8.0 * t89179 + 12.0 * t89097 + 2.0 * t89547 * t258 - t661 * (t89704 + t89712 + t89727 + t89741) * t263 - 2.0 * t89092 - 6.0 * t5059 * t5179;
    (t89749,)
}
