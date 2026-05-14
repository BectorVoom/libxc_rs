//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1129/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1129<F: Float>(t5900: F, t9132: F, t5918: F, t8232: F, t1369: F, t23885: F, t376: F, t1359: F, t7763: F, t23673: F, t95053: F, t23649: F, t23668: F, t1637: F, t5909: F, t7800: F) -> (F, F, F, F, F, F, F, F) {
    let t95293 = t9132 * t5900;
    let t95301 = t8232 * t5918;
    let t95304 = t1369 * t376 * t23885;
    let t95312 = t1359 * t7763;
    let t95320 = t95053 * t23673;
    let t95322 = t23649 * t23668;
    let t95330 = t1369 * t1637 * t5909;
    let t95332 = t1359 * t7800;
    (t95293, t95301, t95304, t95312, t95320, t95322, t95330, t95332)
}
