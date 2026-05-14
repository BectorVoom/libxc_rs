//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1163/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1163<F: Float>(t25462: F, t28993: F, t1466: F, t29041: F, t681: F, t28873: F, t6210: F, t2766: F, t6353: F, t29186: F, t8392: F, t10491: F, t1508: F, t1501: F, t9570: F, t9577: F) -> (F, F, F, F, F, F, F, F) {
    let t112643 = t25462 * t28993 / 27.0;
    let t112647 = t1466 * t681 * t29041 / 9.0;
    let t112649 = t6210 * t28873 / 9.0;
    let t112663 = t2766 * t6353;
    let t112679 = 2.0 / 27.0 * t8392 * t29186;
    let t112680 = t10491 * t1508;
    let t112696 = t1501 * t9570;
    let t112705 = t1501 * t9577;
    (t112643, t112647, t112649, t112663, t112679, t112680, t112696, t112705)
}
