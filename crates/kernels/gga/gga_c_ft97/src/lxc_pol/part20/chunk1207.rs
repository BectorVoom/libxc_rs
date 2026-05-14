//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1207/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1207<F: Float>(t1466: F, t29041: F, t681: F, t28873: F, t6210: F, t43328: F, t7114: F, t10697: F, t2844: F, t7124: F, t2766: F, t6353: F, t15200: F, t6334: F, t29186: F, t8392: F) -> (F, F, F, F, F, F, F) {
    let t112647 = t1466 * t681 * t29041 / 9.0;
    let t112649 = t6210 * t28873 / 9.0;
    let t112654 = t43328 * t7114;
    let t112657 = t10697 * t7124 * t2844;
    let t112663 = t2766 * t6353;
    let t112671 = t6334 * t15200;
    let t112679 = 2.0 / 27.0 * t8392 * t29186;
    (t112647, t112649, t112654, t112657, t112663, t112671, t112679)
}
